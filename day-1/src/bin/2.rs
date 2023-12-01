use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut current_sum = 0;
    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(ip) = line {
                // create an empty vec
                let mut vec: Vec<char> = Vec::new();
                for char in ip.chars() {
                    if char.is_numeric() {
                        vec.push(char);
                    }
                }
                //get first number in vec
                let first = vec[0];
                //get last number in vec
                let last = vec[vec.len() - 1];
                // concatenate first and last
                let first_last = format!("{}{}", first, last);
                // convert to int
                let first_last_int = first_last.parse::<i32>().unwrap();
                // add to current sum
                current_sum = current_sum + first_last_int;
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
