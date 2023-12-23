use crate::utils::read_data;
use std::collections::HashMap;

pub fn solve_puzzle(file_name: &str) -> usize {
    let data = read_data(file_name);

    let map = data
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let start = (0_usize, map[0].iter().position(|c| c == &'.').unwrap());
    let exit = (
        map.len() - 1,
        map[map.len() - 1].iter().position(|c| c == &'.').unwrap(),
    );

    let intersections = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, c)| **c != '#' && is_intersection(&map, i, *j, start, exit))
                .map(|(j, _)| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<(usize, usize)>>();

    // build graph
    let mut graph = HashMap::new();
    for node in intersections.iter() {
        let adjacents = get_adjacents(&map, node, &intersections);
        graph.insert(node, adjacents);
    }

    // Solve with DFS ~ 25s
    let mut result = 0;
    let mut stack = vec![vec![(start, 0)]];
    while let Some(path) = stack.pop() {
        let (last_node, last_distance) = path[path.len() - 1];
        let adjacents = graph.get(&last_node).unwrap();
        for (adjacent, adj_distance) in adjacents.iter() {
            if path.iter().any(|(node, _)| node == adjacent) {
                continue;
            }
            if *adjacent == exit {
                result = result.max(last_distance + adj_distance - 1);
                continue;
            }
            let mut new_path = path.clone();
            new_path.push((*adjacent, last_distance + adj_distance - 1));
            stack.push(new_path);
        }
    }

    result

    // // solve  with dikjstra
    // let mut visited: Vec<(usize, usize)> = Vec::new();
    // let mut djikstra = HashMap::new();
    // djikstra.insert((start, start), -1 as isize);
    // for intersection in intersections.iter() {
    //     for other in intersections.iter() {
    //         if other == &start && intersection == &start {
    //             continue;
    //         }
    //         if  !djikstra.contains_key(&(*other, *intersection)) {
    //             djikstra.insert((*intersection, *other), 0);
    //         }
    //     }
    // }
    // // let mut current_node = start;
    // let keys = djikstra.keys().map(|(a, b)| (*a, *b)).collect::<Vec<((usize, usize), (usize, usize))>>();

    // loop {
    //     let new_dijkstra = djikstra.clone();
    //     let next_data = new_dijkstra
    //     .iter()
    //     .filter(|((_, node), _)| !visited.contains(&node) )
    //     .min_by_key(|(_, distance)| **distance);
    //     if next_data.is_none() {
    //         break;
    //     }
    //     let ((node, _), distance) = next_data.unwrap();
    //     let adjacents = graph.get(&node).unwrap();
    //     for (adjacent, adj_distance) in adjacents.iter() {
    //         if visited.contains(adjacent) {
    //             continue;
    //         }
    //         // if *adjacent == exit {
    //         //     result = result.max(distance + adj_distance - 1);
    //         //     continue;
    //         // }
    //         let new_distance = distance.to_owned() - (*adj_distance as isize);
    //         let key = if keys.contains(&(*adjacent, *node)) {
    //             (*adjacent, *node)
    //         } else {
    //             (*node, *adjacent)
    //         };
    //         println!("dijiksta keys: {:?}", keys);
    //         println!("key: {:?}", key);
    //         let current_distance = djikstra.get(&key).unwrap().to_owned();

    //         if &new_distance < &current_distance {
    //             djikstra.insert(key, new_distance);
    //         }

    //     }
    //     visited.push(node.to_owned());

    // }

    // djikstra.get(&(start, exit)).unwrap().to_owned() as usize
}

fn get_adjacents(
    map: &Vec<Vec<char>>,
    node: &(usize, usize),
    intersections: &[(usize, usize)],
) -> HashMap<(usize, usize), usize> {
    let mut adjacents = HashMap::new();

    let mut visited = Vec::new();
    let mut stack = Vec::new();
    stack.push(vec![*node]);

    while let Some(path) = stack.pop() {
        let next_paths = get_next_paths(map, &path);
        for next_path in next_paths {
            if visited.contains(&next_path) {
                continue;
            }
            visited.push(next_path.clone());
            let last_node = next_path[next_path.len() - 1];
            if intersections.contains(&last_node) {
                adjacents.insert(last_node, next_path.len());
            } else {
                stack.push(next_path);
            }
        }
    }

    adjacents
}

fn get_next_paths(map: &Vec<Vec<char>>, path: &Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
    let (i, j) = path[path.len() - 1];
    let mut next_paths = Vec::new();
    if i > 0 {
        let next_position = (i - 1, j);
        if !path.contains(&next_position) {
            let next_value = map[i - 1][j];
            if next_value != '#' {
                let mut new_path = path.clone();
                new_path.push(next_position);
                next_paths.push(new_path);
            }
        }
    }
    if i < map.len() - 1 {
        let next_position = (i + 1, j);
        if !path.contains(&next_position) {
            let next_value = map[i + 1][j];
            if next_value != '#' {
                let mut new_path = path.clone();
                new_path.push(next_position);
                next_paths.push(new_path);
            }
        }
    }
    if j > 0 {
        let next_position = (i, j - 1);
        if !path.contains(&next_position) {
            let next_value = map[i][j - 1];
            if next_value != '#' {
                let mut new_path = path.clone();
                new_path.push(next_position);
                next_paths.push(new_path);
            }
        }
    }
    if j < map[0].len() - 1 {
        let next_position = (i, j + 1);
        if !path.contains(&next_position) {
            let next_value = map[i][j + 1];
            if next_value != '#' {
                let mut new_path = path.clone();
                new_path.push(next_position);
                next_paths.push(new_path);
            }
        }
    }

    next_paths
}

fn is_intersection(
    map: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    start: (usize, usize),
    exit: (usize, usize),
) -> bool {
    if (i, j) == start || (i, j) == exit {
        return true;
    }
    if map[i][j] == '#' {
        return false;
    }

    let mut directions = Vec::new();
    if i > 0 {
        directions.push((i - 1, j));
    }
    if i < map.len() - 1 {
        directions.push((i + 1, j));
    }
    if j > 0 {
        directions.push((i, j - 1));
    }
    if j < map[0].len() - 1 {
        directions.push((i, j + 1));
    }

    directions
        .iter()
        .filter(|(i, j)| map[*i][*j] != '#')
        .count()
        > 2
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example_data() {
        assert_eq!(154, solve_puzzle("test_data"));
    }

    #[test]
    fn test_solution() {
        assert_eq!(6262, solve_puzzle("input"));
    }
}
