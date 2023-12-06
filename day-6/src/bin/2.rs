use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Race = (isize, isize);
fn main() {
    let mut races: Vec<Race> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        // vector of integers
        for (i, line) in lines.enumerate() {
            if let Ok(row) = line {
                if i == 0 {
                    let strings = row.split_whitespace().collect::<Vec<&str>>();
                    for (i, string) in strings.iter().enumerate() {
                        if i != 0 {
                            races.push((string.parse::<isize>().unwrap(), 0));
                        }
                    }
                } else {
                    let strings = row.split_whitespace().collect::<Vec<&str>>();
                    for (i, string) in strings.iter().enumerate() {
                        if i != 0 {
                            races[i - 1].1 = string.parse::<isize>().unwrap();
                            println!(
                                "{}",
                                races[i - 1].0.to_string() + " " + &races[i - 1].1.to_string()
                            );
                        }
                    }
                }
            }
        }
    }
    let mut multiplier = 1;
    for (duration, record) in races {
        let mut counter = 0;
        for i in 0..duration {
            if (i * (duration - i)) > record {
                counter += 1;
            }
        }
        multiplier *= counter;
    }
    println!("{}", multiplier);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
