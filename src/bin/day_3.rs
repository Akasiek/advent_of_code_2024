use advent_of_code_2024::helpers::get_input_from_file;
use advent_of_code_2024::solutions::day_3::*;

fn main() {
    let input = get_input_from_file(3);
    
    println!("Solution 1: {}", solution_1(&input));
    println!("Solution 2: {}", solution_2(&input));
}