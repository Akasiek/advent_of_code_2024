use std::collections::HashSet;
use itertools::Itertools;

pub fn solution_1(input: &String) -> i32 {
    let coords = get_coords(input);

    get_guard_route(&coords).len() as i32
}

pub fn solution_2(input: &String) -> i32 {
    let coords = get_coords(input);
    let mut sum = 0; 
    
    for (row, col, char) in &coords {
        // Make it loop
        if *char == '.' {
            let new_coords = get_new_coords_with_new_obstruction(&coords, &(*row, *col));
            if check_if_route_is_in_loop(&new_coords) {
                println!("Is in loop");
                sum += 1;
            }
        }
    }
    
    sum
}

fn get_coords(input: &String) -> Vec<(i32, i32, char)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, char)| (row as i32, col as i32, char))
        })
        .collect()
}

fn get_guard_route(coords: &Vec<(i32, i32, char)>) -> HashSet<(i32, i32)> {
    let mut guard_pos = coords.iter().find(|(_, _, char)| *char == '^').map(|(row, col, _)| (*row, *col)).unwrap();
    let mut guard_dir = Direction::North;
    let mut visited: HashSet<_> = HashSet::from([guard_pos.clone()]);
    loop {
        let new_pos = move_guard(coords, &guard_pos, &guard_dir);

        if new_pos.is_none() {
            break;
        } else if new_pos.unwrap() == (-1, -1) {
            guard_dir = match guard_dir {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
        } else {
            let new_pos = new_pos.unwrap();
            visited.insert(new_pos);
            guard_pos = new_pos;
        }
    }

    visited
}

fn check_if_route_is_in_loop(coords: &Vec<(i32, i32, char)>) -> bool {
    let mut guard_pos = coords.iter().find(|(_, _, char)| *char == '^').map(|(row, col, _)| (*row, *col)).unwrap();
    let mut guard_dir = Direction::North;
    let mut visited: Vec<Vec<_>> = Vec::new();
    let mut route: Vec<_> = Vec::from([guard_pos.clone()]);
    loop {
        let new_pos = move_guard(coords, &guard_pos, &guard_dir);
        if new_pos.is_none() {
            break;
        } else if new_pos.unwrap() == (-1, -1) {
            guard_dir = match guard_dir {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
            
            visited.push(route.clone());
            if is_in_loop(visited.clone()) {
                return true;
            }
            
            route = Vec::new();
        } else {
            let new_pos = new_pos.unwrap();
            guard_pos = new_pos;            
            route.push(new_pos);
        }
    }

    false
}

fn move_guard(
    coords: &Vec<(i32, i32, char)>,
    guard_pos: &(i32, i32),
    direction: &Direction,
) -> Option<(i32, i32)> {
    let new_pos = match direction {
        Direction::North => (guard_pos.0 - 1, guard_pos.1),
        Direction::East => (guard_pos.0, guard_pos.1 + 1),
        Direction::West => (guard_pos.0, guard_pos.1 - 1),
        Direction::South => (guard_pos.0 + 1, guard_pos.1),
    };

    match find_char_by_pos(coords, new_pos) {
        None => None,
        Some('#') => Some((-1, -1)),
        _ => Some(new_pos),
    }
}

fn find_char_by_pos(coords: &Vec<(i32, i32, char)>, pos: (i32, i32)) -> Option<char> {
    coords
        .iter()
        .find(|(row, col, _)| pos.0 == *row && pos.1 == *col)
        .map(|(_, _, char)| *char)
}

fn is_in_loop(mut visited: Vec<Vec<(i32, i32)>>) -> bool {
    let last = visited.pop().unwrap();    
    visited.contains(&last)
}

fn get_new_coords_with_new_obstruction(coords: &Vec<(i32, i32, char)>, obstruction_pos: &(i32,i32)) -> Vec<(i32, i32, char)> {
    coords.iter().map(|(row,col,char)| {
        if obstruction_pos.0 == *row && obstruction_pos.1 == *col {
            (*row, *col, '#')
        } else {
            (*row, *col, *char)
        }
    }).collect()
}


enum Direction {
    North,
    East,
    West,
    South,
}
