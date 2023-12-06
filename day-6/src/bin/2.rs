use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Race = (isize, isize);
fn main() {
    let mut race: Race = (0, 0);
    let mut accumulated_string_distance = String::new();
    let mut accumulated_string_time = String::new();
    if let Ok(lines) = read_lines("./input.txt") {
        // vector of integers
        for (i, line) in lines.enumerate() {
            if let Ok(row) = line {
                if i == 0 {
                    let strings = row.split_whitespace().collect::<Vec<&str>>();
                    for (i, string) in strings.iter().enumerate() {
                        if i != 0 {
                            accumulated_string_time.push_str(string);
                        }
                    }
                    race.0 = accumulated_string_time.parse::<isize>().unwrap();
                } else {
                    let strings = row.split_whitespace().collect::<Vec<&str>>();
                    for (i, string) in strings.iter().enumerate() {
                        if i != 0 {
                            accumulated_string_distance.push_str(string);
                        }
                    }
                    race.1 = accumulated_string_distance.parse::<isize>().unwrap();
                }
            }
        }
    }
    let mut counter = 0;

    // solve quadratic equation
    // - x^2 + x.race.0 - (1 + race.1) = 0
    // x = (-b +- sqrt(b^2 - 4ac)) / 2a
    /*
    let first_root = (-race.0 as f64
        + f64::sqrt(race.0.pow(2) as f64 + 4 as f64 * (-1 + race.1) as f64))
        / -2 as f64;
    let second_root = (-race.0 as f64
        - f64::sqrt(race.0.pow(2) as f64 + 4 as f64 * (-1 + race.1) as f64))
        / -2 as f64; */

    println!("{}", race.0);
    println!("{}", race.1);
    println!("{}", 20000000 * (race.0 - 20000000) > race.1)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
