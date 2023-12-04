use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                let mut card_sum = 0;
                let numbers = row.split(':').collect::<Vec<&str>>()[1];
                let winning_numbers = numbers.split('|').collect::<Vec<&str>>()[0];
                let own_numbers = numbers.split('|').collect::<Vec<&str>>()[1];

                for number in own_numbers.split_whitespace() {
                    for winning_number in winning_numbers.split_whitespace() {
                        if number == winning_number {
                            if card_sum == 0 {
                                card_sum = 1
                            } else {
                                card_sum = card_sum * 2;
                            }
                        }
                    }
                }
                sum += card_sum;
            }
        }
        println!("{}", sum)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
