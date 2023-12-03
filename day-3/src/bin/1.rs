use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Coordinate = (usize, usize);
//coordinate start and end of number from first digit to last digit
type Coordinates = (Coordinate, Coordinate);

fn main() {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut sum = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                map.push(row.chars().collect());
            }
        }
    }
    let mut numbers_and_coordinates = Vec::<(i32, Coordinates)>::new();
    for i in 0..map.len() {
        let mut unfinished_number = String::new();
        let mut start_coordinate: Coordinate = (0, 0);
        let mut end_coordinate: Coordinate = (0, 0);
        for j in 0..map.len() {
            if map[i][j].is_numeric() {
                unfinished_number.push(map[i][j]);
                if unfinished_number.len() == 1 {
                    start_coordinate = (i, j);
                }
                end_coordinate = (i, j);
                if j == (map.len() - 1) {
                    let number = unfinished_number.parse::<i32>().unwrap();
                    unfinished_number = String::new();
                    numbers_and_coordinates.push((number, (start_coordinate, end_coordinate)));
                }
            } else if unfinished_number.len() > 0 {
                let number = unfinished_number.parse::<i32>().unwrap();
                unfinished_number = String::new();
                numbers_and_coordinates.push((number, (start_coordinate, end_coordinate)));
            }
        }
    }
    for number_and_coordinates in numbers_and_coordinates {
        let number = number_and_coordinates.0;
        let coordinates = number_and_coordinates.1;
        let start_coordinate = coordinates.0;
        let end_coordinate = coordinates.1;
        let mut has_adjacent_symbol = false;

        let can_go_left = start_coordinate.1 as i32 - 1 >= 0;
        let can_go_right = end_coordinate.1 + 1 < map.len();
        let can_go_up = start_coordinate.0 as i32 - 1 >= 0;
        let can_go_down = start_coordinate.0 + 1 < map.len();
        if can_go_left {
            if can_go_up {
                let element = map[start_coordinate.0 - 1][start_coordinate.1 - 1];
                if !element.is_numeric() && element != '.' {
                    has_adjacent_symbol = true;
                }
            }
            if can_go_down {
                let element = map[start_coordinate.0 + 1][start_coordinate.1 - 1];
                if !element.is_numeric() && element != '.' {
                    has_adjacent_symbol = true;
                }
            }
            let element = map[start_coordinate.0][start_coordinate.1 - 1];
            if !element.is_numeric() && element != '.' {
                has_adjacent_symbol = true;
            }
        }

        if can_go_right {
            if can_go_up {
                let element = map[end_coordinate.0 - 1][end_coordinate.1 + 1];
                if !element.is_numeric() && element != '.' {
                    has_adjacent_symbol = true;
                }
            }
            if can_go_down {
                let element = map[end_coordinate.0 + 1][end_coordinate.1 + 1];
                if !element.is_numeric() && element != '.' {
                    has_adjacent_symbol = true;
                }
            }
            let element = map[end_coordinate.0][end_coordinate.1 + 1];
            if !element.is_numeric() && element != '.' {
                has_adjacent_symbol = true;
            }
        }

        if can_go_up {
            for j in start_coordinate.1..=end_coordinate.1 {
                let element = map[start_coordinate.0 - 1][j];
                if !element.is_numeric() && element != '.' {
                    has_adjacent_symbol = true;
                }
            }
        }

        if can_go_down {
            for j in start_coordinate.1..=end_coordinate.1 {
                let element = map[start_coordinate.0 + 1][j];
                if !element.is_numeric() && element != '.' {
                    has_adjacent_symbol = true;
                }
            }
        }

        if has_adjacent_symbol {
            sum += number;
        }
    }
    println!("{}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
