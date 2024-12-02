use crate::helpers::get_input_from_file;

mod helpers;
mod solutions;

fn main() {
    // let input_one = get_input_from_file(1);
    // println!("{}", solutions::day_1::solution_1(&input_one));
    // println!("{}", solutions::day_1::solution_2(&input_one));

    let input_two_test = String::from(
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
    );
    let input_two = get_input_from_file(2);
    println!("{}", solutions::day_2::solution_1(&input_two));
    println!("{}", solutions::day_2::solution_2(&input_two));
}
