use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut current_sum = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                // create an empty vec
                let mut numbers_and_positions: Vec<(i32, i32)> = Vec::new();

                let number_names = [
                    ("zero", 0),
                    ("one", 1),
                    ("two", 2),
                    ("three", 3),
                    ("four", 4),
                    ("five", 5),
                    ("six", 6),
                    ("seven", 7),
                    ("eight", 8),
                    ("nine", 9),
                ];
                for (number_name, number) in number_names {
                    let option_position = ip.find(number_name);
                    if let Some(position) = option_position {
                        //cast position to i32
                        let position = position as i32;
                        numbers_and_positions.push((number, position));
                    }
                }

                for (i, char) in ip.chars().enumerate() {
                    if char.is_numeric() {
                        numbers_and_positions.push((char.to_digit(10).unwrap() as i32, i as i32));
                    }
                }
                //find the number with the lowest position
                let mut lowest_position_number = 0;
                let mut max_position_number = 0;
                let mut lowest_position = 1000;
                let mut max_position = 0;
                for (number, position) in numbers_and_positions {
                    if position < lowest_position {
                        lowest_position = position;
                        lowest_position_number = number;
                    }
                    if position > max_position {
                        max_position = position;
                        max_position_number = number;
                    }
                }
                // concatenate first and last
                let first_last = format!("{}{}", lowest_position_number, max_position_number);
                // add to current sum
                current_sum = current_sum + first_last.parse::<i32>().unwrap();
            }
        }
        println!("Current sum is: {}", current_sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/*
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
} */
