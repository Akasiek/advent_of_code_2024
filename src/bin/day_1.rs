use advent_of_code_2024::helpers::get_input_from_file;
use advent_of_code_2024::solutions::day_1;

fn main() {
    let input_one = get_input_from_file(1);
    println!("Solution 1: {}", day_1::solution_1(&input_one));
    println!("Solution 2: {}", day_1::solution_2(&input_one));
}