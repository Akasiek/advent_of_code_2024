use crate::helpers::get_input_from_file;

mod helpers;
mod solutions;

fn main() {
    let input_one = get_input_from_file(1);
    println!("{}", solutions::day_1::solution_1(&input_one));
    println!("{}", solutions::day_1::solution_2(&input_one));
}
