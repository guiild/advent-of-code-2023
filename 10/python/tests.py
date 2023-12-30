from utils import Grid, Pipe
from day10 import solve_part1, solve_part2

def read_example(text):
    grid = Grid()
    lines = [l for l in text.split("\n") if l.strip()]
    for i, line in enumerate(lines):
        line_list = []  # list of pipes or tuples of the form (ch, i, j)
        for j, ch in enumerate(line.strip()):
            if ch not in [".", "S"]:
                pipe = Pipe((i,j), ch)
                line_list.append(pipe)
            else:
                line_list.append((ch, i, j))
                if ch == "S":
                    starting_i, starting_j = (i, j)
        grid.append_row(line_list)
    grid.pad()
    starting_i += 1
    starting_j += 1
    return grid, starting_i, starting_j

def test_part1_example1():
    text = """
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
"""
    grid, starting_i, starting_j = read_example(text)
    # part 1
    loop, ans_part1 = solve_part1(grid, starting_i, starting_j)
    assert ans_part1 == 4
    # part 2: 1
    grid, ans_part2 = solve_part2(grid, starting_i, starting_j, loop)
    assert ans_part2 == 1


def test_part1_example2():
    text = """
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
"""
    grid, starting_i, starting_j = read_example(text)
    # part 1
    loop, ans_part1 = solve_part1(grid, starting_i, starting_j)
    assert ans_part1 == 8
    # part 2: 1
    grid, ans_part2 = solve_part2(grid, starting_i, starting_j, loop)
    assert ans_part2 == 1

def test_part2_example1():
    text = """
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"""
    grid, starting_i, starting_j = read_example(text)
    # part 2: 4
    loop, _ = solve_part1(grid, starting_i, starting_j)
    grid, ans_part2 = solve_part2(grid, starting_i, starting_j, loop)
    assert ans_part2 == 4


def test_part2_example2():
    text = """
..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
"""
    grid, starting_i, starting_j = read_example(text)
    # part 2: 4
    loop, _ = solve_part1(grid, starting_i, starting_j)
    grid, ans_part2 = solve_part2(grid, starting_i, starting_j, loop)
    assert ans_part2 == 4


def test_part2_example3():
    text = """
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"""
    grid, starting_i, starting_j = read_example(text)
    # part 2: 8
    loop, _ = solve_part1(grid, starting_i, starting_j)
    grid, ans_part2 = solve_part2(grid, starting_i, starting_j, loop)
    assert ans_part2 == 8

def test_part2_example4():
    text = """
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"""
    grid, starting_i, starting_j = read_example(text)
    # part 2: 10
    loop, _ = solve_part1(grid, starting_i, starting_j)
    grid, ans_part2 = solve_part2(grid, starting_i, starting_j, loop)
    assert ans_part2 == 10
