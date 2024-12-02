use advent_of_code_2024::helpers::get_input_from_file;
use advent_of_code_2024::solutions;

fn main() {
    let input_two = get_input_from_file(2);
    println!("Solution 1: {}", solutions::day_2::solution_1(&input_two));
    println!("Solution 2: {}", solutions::day_2::solution_2(&input_two));
}