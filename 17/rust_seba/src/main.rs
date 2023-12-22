mod part1;
mod part1_first_try;
mod part2;
mod utils;

fn main() {
    println!("The result for part 1 is {}", part1::solve_puzzle("input"));
    println!(
        "The result for part 1 (on test data - First try) is {}",
        part1_first_try::solve_puzzle("test_data")
    );
    println!("The result for part 2 is {}", part2::solve_puzzle("input"));
}
