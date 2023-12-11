use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut board: Vec<Vec<char>> = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                board.push(row.chars().collect::<Vec<char>>());
            }
        }
    };
    let mut s_coordinates = (0, 0);
    for (x, row) in board.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if col == &'S' {
                s_coordinates = (x, y);
            }
        }
    }
    let mut back = false;
    let mut count_steps = 0;
    let mut previous_coordinates = (s_coordinates.0, s_coordinates.1);
    let mut current_coordinates = (s_coordinates.0, s_coordinates.1 + 1);
    while !back {
        count_steps += 1;
        let current_char = board[current_coordinates.0][current_coordinates.1];
        let mut next_coordinates = (0, 0);
        if current_coordinates == s_coordinates {
            back = true;
        }
        if current_char == '|' {
            if current_coordinates.0 < previous_coordinates.0 {
                next_coordinates = (current_coordinates.0 - 1, current_coordinates.1);
            } else {
                next_coordinates = (current_coordinates.0 + 1, current_coordinates.1);
            }
        } else if current_char == '-' {
            if current_coordinates.1 < previous_coordinates.1 {
                next_coordinates = (current_coordinates.0, current_coordinates.1 - 1);
            } else {
                next_coordinates = (current_coordinates.0, current_coordinates.1 + 1);
            }
        } else if current_char == 'L' {
            if current_coordinates.0 > previous_coordinates.0 {
                next_coordinates = (current_coordinates.0, current_coordinates.1 + 1);
            } else {
                next_coordinates = (current_coordinates.0 - 1, current_coordinates.1);
            }
        } else if current_char == 'J' {
            if current_coordinates.0 > previous_coordinates.0 {
                next_coordinates = (current_coordinates.0, current_coordinates.1 - 1);
            } else {
                next_coordinates = (current_coordinates.0 - 1, current_coordinates.1);
            }
        } else if current_char == '7' {
            if current_coordinates.0 < previous_coordinates.0 {
                next_coordinates = (current_coordinates.0, current_coordinates.1 - 1);
            } else {
                next_coordinates = (current_coordinates.0 + 1, current_coordinates.1);
            }
        } else if current_char == 'F' {
            if current_coordinates.0 < previous_coordinates.0 {
                next_coordinates = (current_coordinates.0, current_coordinates.1 + 1);
            } else {
                next_coordinates = (current_coordinates.0 + 1, current_coordinates.1);
            }
        }
        previous_coordinates = current_coordinates;
        current_coordinates = next_coordinates;
    }
    println!("Steps: {}", count_steps);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
