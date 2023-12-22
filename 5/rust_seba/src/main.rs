mod part1;
mod part2;
mod part2_optimized;
mod utils;

fn main() {
    println!("The result for part 1 is {}", part1::solve_puzzle("input"));
    println!(
        "The result for part 2 is {}",
        part2_optimized::solve_puzzle("input")
    );
}
