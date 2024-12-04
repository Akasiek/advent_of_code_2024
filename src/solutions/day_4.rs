pub fn solution_1(input: &String) -> i32 {
    let letters = get_letters_by_coords(input);

    get_xmas_count(letters)
}

pub fn solution_2(input: &String) -> i32 {
    let letters = get_letters_by_coords(input);

    get_x_mas_count(letters)
}

fn get_letters_by_coords(input: &String) -> Vec<(char, i32, i32)> {
    let mut letters: Vec<(char, i32, i32)> = Vec::new();

    for (row, line) in input.split("\n").enumerate() {
        for (col, letter) in line.chars().enumerate() {
            letters.push((letter, row as i32, col as i32));
        }
    }

    letters
}

fn get_xmas_count(letters: Vec<(char, i32, i32)>) -> i32 {
    letters
        .iter()
        .filter(|letter| letter.0 == 'X')
        .map(|letter| get_x_xmases(letter, &letters))
        .sum()
}

fn get_x_xmases(x: &(char, i32, i32), letters: &Vec<(char, i32, i32)>) -> i32 {
    let mut possible_xmases: Vec<Direction> = Vec::new();

    // Top
    if get_letter_by_coords(letters, x.1 - 1, x.2).unwrap_or('0') == 'M' {
        possible_xmases.push(Direction::Top);
    }

    // TopRight
    if get_letter_by_coords(letters, x.1 - 1, x.2 + 1).unwrap_or('0') == 'M' {
        possible_xmases.push(Direction::TopRight);
    }

    // Right
    if get_letter_by_coords(letters, x.1, x.2 + 1).unwrap_or('0') == 'M' {
        possible_xmases.push(Direction::Right);
    }

    // BottomRight
    if get_letter_by_coords(letters, x.1 + 1, x.2 + 1).unwrap_or('0') == 'M' {
        possible_xmases.push(Direction::BottomRight);
    }

    // Bottom
    if get_letter_by_coords(letters, x.1 + 1, x.2).unwrap_or('0') == 'M' {
        possible_xmases.push(Direction::Bottom);
    }

    // BottomLeft
    if get_letter_by_coords(letters, x.1 + 1, x.2 - 1).unwrap_or('0') == 'M' {
        possible_xmases.push(Direction::BottomLeft);
    }

    // Left
    if get_letter_by_coords(letters, x.1, x.2 - 1).unwrap_or('0') == 'M' {
        possible_xmases.push(Direction::Left);
    }

    // TopLeft
    if get_letter_by_coords(letters, x.1 - 1, x.2 - 1).unwrap_or('0') == 'M' {
        possible_xmases.push(Direction::TopLeft);
    }

    // println!("{:?}", check_possible_xmas(x, letters, &Direction::Bottom));

    possible_xmases
        .iter()
        .filter(|d| check_possible_xmas(x, letters, d))
        .count() as i32
}

fn check_possible_xmas(
    x: &(char, i32, i32),
    letters: &Vec<(char, i32, i32)>,
    direction: &Direction,
) -> bool {
    let mut is_xmas = true;
    let chars = ['M', 'A', 'S'];

    match direction {
        Direction::Top => {
            for (index, char) in chars.iter().enumerate() {
                if get_letter_by_coords(letters, x.1 - (index as i32 + 1), x.2).unwrap_or('0')
                    != *char
                {
                    is_xmas = false;
                    return is_xmas;
                }
            }
        }
        Direction::TopRight => {
            for (index, char) in chars.iter().enumerate() {
                if get_letter_by_coords(letters, x.1 - (index as i32 + 1), x.2 + (index as i32 + 1))
                    .unwrap_or('0')
                    != *char
                {
                    is_xmas = false;
                    return is_xmas;
                }
            }
        }
        Direction::Right => {
            for (index, char) in chars.iter().enumerate() {
                if get_letter_by_coords(letters, x.1, x.2 + (index as i32 + 1)).unwrap_or('0')
                    != *char
                {
                    is_xmas = false;
                    return is_xmas;
                }
            }
        }
        Direction::BottomRight => {
            for (index, char) in chars.iter().enumerate() {
                if get_letter_by_coords(letters, x.1 + (index as i32 + 1), x.2 + (index as i32 + 1))
                    .unwrap_or('0')
                    != *char
                {
                    is_xmas = false;
                    return is_xmas;
                }
            }
        }
        Direction::Bottom => {
            for (index, char) in chars.iter().enumerate() {
                if get_letter_by_coords(letters, x.1 + (index as i32 + 1), x.2).unwrap_or('0')
                    != *char
                {
                    is_xmas = false;
                    return is_xmas;
                }
            }
        }
        Direction::BottomLeft => {
            for (index, char) in chars.iter().enumerate() {
                if get_letter_by_coords(letters, x.1 + (index as i32 + 1), x.2 - (index as i32 + 1))
                    .unwrap_or('0')
                    != *char
                {
                    is_xmas = false;
                    return is_xmas;
                }
            }
        }
        Direction::Left => {
            for (index, char) in chars.iter().enumerate() {
                if get_letter_by_coords(letters, x.1, x.2 - (index as i32 + 1)).unwrap_or('0')
                    != *char
                {
                    is_xmas = false;
                    return is_xmas;
                }
            }
        }
        Direction::TopLeft => {
            for (index, char) in chars.iter().enumerate() {
                if get_letter_by_coords(letters, x.1 - (index as i32 + 1), x.2 - (index as i32 + 1))
                    .unwrap_or('0')
                    != *char
                {
                    is_xmas = false;
                    return is_xmas;
                }
            }
        }
    };

    is_xmas
}

fn get_x_mas_count(letters: Vec<(char, i32, i32)>) -> i32 {
    letters
        .iter()
        .filter(|letter| letter.0 == 'A')
        .filter(|letter| has_a_x_mas(letter, &letters))
        .count() as i32
}

fn has_a_x_mas(a: &(char, i32, i32), letters: &Vec<(char, i32, i32)>) -> bool {
    let top_left = get_letter_by_coords(letters, a.1 - 1, a.2 - 1).unwrap_or('0');
    let top_right = get_letter_by_coords(letters, a.1 - 1, a.2 + 1).unwrap_or('0');
    let bottom_left = get_letter_by_coords(letters, a.1 + 1, a.2 - 1).unwrap_or('0');
    let bottom_right = get_letter_by_coords(letters, a.1 + 1, a.2 + 1).unwrap_or('0');

    if top_left == 'M' && top_right == 'M' && bottom_left == 'S' && bottom_right == 'S' {
        true
    } else if top_left == 'S' && top_right == 'M' && bottom_left == 'S' && bottom_right == 'M' {
        true
    } else if top_left == 'S' && top_right == 'S' && bottom_left == 'M' && bottom_right == 'M' {
        true
    } else if top_left == 'M' && top_right == 'S' && bottom_left == 'M' && bottom_right == 'S' {
        true
    } else {
        false
    }
}

fn get_letter_by_coords(letters: &Vec<(char, i32, i32)>, row: i32, col: i32) -> Option<char> {
    let letter = letters.iter().filter(|l| l.1 == row && l.2 == col).next();

    match letter.is_some() {
        true => Some(letter.unwrap().0),
        false => None,
    }
}

#[derive(Debug)]
enum Direction {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}
