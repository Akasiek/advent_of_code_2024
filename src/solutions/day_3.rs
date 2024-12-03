use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone)]
struct Mul {
    where_is: i32,
    left: i32,
    right: i32,
}

#[derive(PartialEq)]
enum DoOrDont {
    Do,
    Dont,
}

pub fn solution_1(input: &String) -> i32 {
    let mut sum = 0;
    let re = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    for (_, [left, right]) in re.captures_iter(input).map(|c| c.extract()) {
        let res = multiple_str(left, right);

        if res.is_some() {
            sum += res.unwrap();
        }
    }

    sum
}

pub fn solution_2(input: &String) -> i32 {
    let mut muls: Vec<Mul> = Vec::new();
    let mul_regex = Regex::new(r"mul\((\d*),(\d*)\)").unwrap();

    for captures in mul_regex.captures_iter(input) {
        let mut iterator = captures.iter();
        let capture = iterator.next().unwrap().unwrap();
        let where_is = capture.end() as i32;

        let left = iterator.next().unwrap().unwrap().as_str();
        let right = iterator.next().unwrap().unwrap().as_str();

        muls.push(Mul {
            where_is,
            left: left.parse::<i32>().unwrap(),
            right: right.parse::<i32>().unwrap(),
        });
    }

    let do_dont_regex = Regex::new(r"(do\(\))|(don't\(\))").unwrap();
    let mut do_dont_map: HashMap<i32, DoOrDont> = HashMap::new();
    for captures in do_dont_regex.captures_iter(input) {
        let capture = captures.iter().next().unwrap().unwrap();
        let is_do = match capture.as_str().len() {
            4 => DoOrDont::Do,
            _ => DoOrDont::Dont,
        };

        do_dont_map.insert(capture.end() as i32, is_do);
    }

    remove_disabled_muls(muls.clone(), do_dont_map)
        .iter()
        .map(|mul| mul.left * mul.right)
        .sum()
}

fn multiple_str(left: &str, right: &str) -> Option<i32> {
    let left_int = left.parse::<i32>();
    let right_int = right.parse::<i32>();

    if left_int.is_err() || right_int.is_err() {
        return None;
    }

    Some(left_int.unwrap() * right_int.unwrap())
}

fn remove_disabled_muls(mut muls: Vec<Mul>, do_dont_map: HashMap<i32, DoOrDont>) -> Vec<Mul> {
    let mut to_delete: Vec<usize> = Vec::new();

    let mut do_dont_sorted: Vec<_> = do_dont_map.iter().collect();
    do_dont_sorted.sort_by_key(|&(key, _)| key);

    for (do_dont1, do_dont2) in do_dont_sorted.iter().tuple_windows() {
        if *do_dont1.1 == DoOrDont::Dont {
            let range = do_dont1.0.clone()..=do_dont2.0.clone();
            for (index, mul) in muls.iter().enumerate() {
                if range.contains(&mul.where_is) {
                    to_delete.push(index);
                }
            }
        }
    }

    // Remove muls after last don't
    let last_do_dont = do_dont_sorted.get(do_dont_sorted.len() - 1).unwrap();
    if *last_do_dont.1 == DoOrDont::Dont {
        for (index, mul) in muls.iter().enumerate() {
            if mul.where_is > last_do_dont.0.clone() {
                to_delete.push(index);
            }
        }
    }

    to_delete.reverse();
    to_delete.iter().for_each(|index| {
        muls.remove(*index);
    });

    muls
}
