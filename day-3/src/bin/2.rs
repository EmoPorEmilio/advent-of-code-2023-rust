use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Coordinate = (usize, usize);
//coordinate start and end of number from first digit to last digit
type Coordinates = (Coordinate, Coordinate);

fn main() {
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut sum_gears = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                map.push(row.chars().collect());
            }
        }
    }
    let mut gear_coordinates = Vec::<Coordinate>::new();
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
            if map[i][j] == '*' {
                gear_coordinates.push((i, j));
            }
        }
    }
    for gear in gear_coordinates {
        let mut count_adjacent_numbers = 0;
        let mut product_adjacent_numbers = 1;
        let (gear_x, gear_y) = gear;
        for number_and_coordinates in &numbers_and_coordinates {
            let (number, ((start_coordinate_x, start_coordinate_y), (_, end_coordinate_y))) =
                number_and_coordinates;
            let is_adjacent_row = gear_x.abs_diff(*start_coordinate_x) <= 1;
            let is_adjacent_column = gear_y.abs_diff(*start_coordinate_y) <= 1
                || gear_y.abs_diff(*end_coordinate_y) <= 1;
            if is_adjacent_column && is_adjacent_row {
                count_adjacent_numbers += 1;
                product_adjacent_numbers *= number;
            }
        }
        if count_adjacent_numbers == 2 {
            sum_gears += product_adjacent_numbers;
        }
    }
    println!("{}", sum_gears);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
