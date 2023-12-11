use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut histories: Vec<Vec<isize>> = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        for (i, line) in lines.enumerate() {
            if let Ok(row) = line {
                histories.push(
                    row.split_whitespace()
                        .map(|x| x.parse::<isize>().unwrap())
                        .collect::<Vec<isize>>(),
                );
            }
        }
    };
    let mut sum_extrapolated_values = 0;
    for history in histories {
        let mut differences: Vec<Vec<isize>> = vec![];
        let mut found_zeroes = false;
        let mut counter_steps = 0;
        differences.push(history.clone());
        while !found_zeroes {
            found_zeroes = true;
            if counter_steps == 0 {
                let difference = history
                    .windows(2)
                    .map(|x| {
                        if found_zeroes && x[1] - x[0] != 0 {
                            found_zeroes = false;
                        }
                        x[1] - x[0]
                    })
                    .collect::<Vec<isize>>();
                differences.push(difference);
                counter_steps += 1;
            } else {
                found_zeroes = true;
                let prev_difference = differences[differences.len() - 1].clone();
                let difference = prev_difference
                    .windows(2)
                    .map(|x| {
                        if found_zeroes && x[1] - x[0] != 0 {
                            found_zeroes = false;
                        }
                        x[1] - x[0]
                    })
                    .collect::<Vec<isize>>();
                differences.push(difference);
                counter_steps += 1;
            }
        }
        let mut i = differences.len() - 1;
        let mut prev_last_digit = 0;
        while i > 0 {
            let len = differences[i].len() - 1;
            let last_digit = differences[i][len];
            prev_last_digit = prev_last_digit + last_digit;
            i -= 1;
        }

        let len = differences[0].len() - 1;
        let last_digit = differences[0][len];
        prev_last_digit = prev_last_digit + last_digit;
        println!("Prev last digit: {:?}", prev_last_digit);
        sum_extrapolated_values += prev_last_digit;
        println!("Sum of extrapolated values: {:?}", sum_extrapolated_values)
    }
    //println!("Sum of extrapolated values: {:?}", sum_extrapolated_values);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
