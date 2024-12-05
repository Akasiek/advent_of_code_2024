use itertools::Itertools;
use std::collections::HashMap;

pub fn solution_1(input: &String) -> i32 {
    let (rules, updates) = input.splitn(2, "\n\n").collect_tuple().unwrap();
    let rules_map = get_rules_map(rules);
    let (good_updates, _) = get_updates_by_correctness(updates, &rules_map);

    good_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn solution_2(input: &String) -> i32 {
    let (rules, updates) = input.splitn(2, "\n\n").collect_tuple().unwrap();
    let rules_map = get_rules_map(rules);
    let (_, bad_updates) = get_updates_by_correctness(updates, &rules_map);

    let fixed_bad_updates = get_fixed_bad_updates(bad_updates, &rules_map);

    fixed_bad_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum()
}

fn get_rules_map(rules: &str) -> HashMap<i32, Vec<i32>> {
    rules.lines().fold(HashMap::new(), |mut map, rule| {
        let (left, right) = rule
            .split('|')
            .map(|i| i.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();

        map.entry(left).or_insert_with(Vec::new).push(right);
        map
    })
}

fn get_updates_by_correctness(
    updates: &str,
    rules_map: &HashMap<i32, Vec<i32>>,
) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut good_updates: Vec<Vec<i32>> = Vec::new();
    let mut bad_updates: Vec<Vec<i32>> = Vec::new();

    for update in updates.lines() {
        let numbers: Vec<i32> = update
            .split(",")
            .map(|i| i.parse::<i32>().unwrap())
            .collect();

        if check_lines_with_rules(&numbers, &rules_map) {
            good_updates.push(numbers)
        } else {
            bad_updates.push(numbers)
        }
    }

    (good_updates, bad_updates)
}

fn check_lines_with_rules(numbers: &Vec<i32>, rules_map: &HashMap<i32, Vec<i32>>) -> bool {
    for (index, number) in numbers.iter().enumerate().skip(1) {
        let number_rules = rules_map.get(number).and_then(|rules| {
            (0..index).find(|&i| rules.contains(&numbers[i]))
        });
    
        if number_rules.is_none() {
            continue;
        } else {
            return false;
        }
    }
    
    true
}

fn get_fixed_bad_updates(
    mut bad_updates: Vec<Vec<i32>>,
    rules_map: &HashMap<i32, Vec<i32>>,
) -> Vec<Vec<i32>> {
    for numbers in &mut bad_updates {
        while let Some((i, j)) = numbers
            .iter()
            .enumerate()
            .skip(1)
            .find_map(|(index, number)| {
                rules_map.get(number).and_then(|rules| {
                    (0..index)
                        .find(|&i| rules.contains(&numbers[i]))
                        .map(|i| (index, i))
                })
            })
        {
            numbers.swap(i, j);
        }
    }
    bad_updates
}
