use std::collections::HashMap;
use std::process::exit;

fn get_two_vectors(input: &String) -> (Vec<i32>, Vec<i32>) {
    let mut first_vec: Vec<i32> = Vec::new();
    let mut second_vec: Vec<i32> = Vec::new();

    for line in input.split("\n") {
        let two = line.split("   ").collect::<Vec<&str>>();

        if two.iter().count() != 2 {
            println!("Input error!");
            exit(-1);
        }

        let first = two.get(0).unwrap().parse::<i32>().unwrap();
        let second = two.get(1).unwrap().parse::<i32>().unwrap();

        first_vec.push(first);
        second_vec.push(second);
    }

    (first_vec, second_vec)
}

pub fn solution_1(input: &String) -> i32 {
    let (mut first_vec, mut second_vec) = get_two_vectors(input);

    first_vec.sort();
    second_vec.sort();

    let mut sum = 0;

    for (index, first) in first_vec.iter().enumerate() {
        let index_sum = first - second_vec.get(index).unwrap();
        sum += index_sum.abs();
    }

    sum
}

pub fn solution_2(input: &String) -> i32 {
    let (mut first_vec, mut second_vec) = get_two_vectors(input);

    first_vec.sort();
    second_vec.sort();

    let mut sims = HashMap::new();

    for first in &first_vec {
        if sims.contains_key(first) {
            continue;
        }

        let mut sim_score = 0;

        for second in &mut second_vec {
            if second == first {
                sim_score += 1;
            }
        }

        if (sim_score == 0) {
            continue;
        }
        
        sims.insert(first, sim_score);
    }

    let mut sims_sum: i32 = 0;

    for first in &first_vec {
        if sims.contains_key(first) {
            sims_sum += first * sims.get(first).unwrap();
        }
    }

    sims_sum
}
