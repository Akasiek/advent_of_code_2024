use itertools::Itertools;

pub fn solution_1(input: &String) -> i32 {
    let mut safe_count = 0;

    for line in input.split("\n") {
        let report: Report = line.split(" ").map(|i| i.parse::<i32>().unwrap()).collect();        
        if check_safety(&report) {
            safe_count += 1;
        }
    }

    safe_count
}

pub fn solution_2(input: &String) -> i32 {
    let mut safe_count = 0;

    for line in input.split("\n") {
        let report: Report = line.split(" ").map(|i| i.parse::<i32>().unwrap()).collect();

        if !check_safety(&report) {
            for index in 0..report.len() {
                let mut report_without_index: Report = report.clone();
                report_without_index.remove(index);
                
                if check_safety(&report_without_index) {
                    safe_count += 1;
                    break;
                }
            }
        } else {
            safe_count += 1;
        }
    }

    safe_count
}

fn check_safety(report: &Report) -> bool {
    let mut direction: Option<i32> = None;

    for (a, b) in report.iter().tuple_windows() {
        let diff = a - b;
        let sign = diff.signum();

        match sign {
            -1 => match direction {
                Some(1) => return false,
                Some(-1) => if !(1..=3).contains(&diff.abs()) {
                    return false;
                } else {
                    continue;
                }
                None => if !(1..=3).contains(&diff.abs()) {
                    return false;
                } else {
                    direction = Some(-1);
                    continue;
                },
                _ => panic!("Wtf?!")
            },
            1 => match direction {
                Some(1) => if !(1..=3).contains(&diff.abs()) {
                    return false;
                } else {
                    continue;
                },
                Some(-1) =>  return false,
                None => if !(1..=3).contains(&diff.abs()) {
                    return false;
                } else {
                    direction = Some(1);
                    continue;
                }
                _ => panic!("Wtf?!")
            },
            0 => return false,
            _ => panic!("Wtf?!")
        }
    }
    
    true
}

type Report = Vec<i32>;