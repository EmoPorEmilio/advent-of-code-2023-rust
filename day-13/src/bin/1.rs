use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut boards: Vec<Vec<Vec<char>>> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        let mut current_board = Vec::new();
        for (i, line) in lines.enumerate() {
            if let Ok(row) = line {
                if row.is_empty() {
                    boards.push(current_board);
                    current_board = Vec::new();
                } else {
                    current_board.push(row.chars().collect());
                }
            }
        }
    };
    for board in boards {
        let mut column_numbers: Vec<usize> = vec![];
        let mut row_numbers: Vec<usize> = vec![];
        for row in board {
            let mut current_number = 0;
            for (index, &c) in row.iter().enumerate() {
                if c == '.' {
                    current_number += 2 * ((2 as usize).pow(index as u32));
                } else if c == '#' {
                    current_number += 1 * ((2 as usize).pow(index as u32));
                }
            }
            row_numbers.push(current_number);
            println!();
        }
        for j in 0..board.len() {
            for i in 0..board[j].len() {
                let mut current_number = 0;
                for j in 0..board.len() {
                    if board[j][i] == '.' {
                        current_number += 2 * ((2 as usize).pow(j as u32));
                    } else if board[j][i] == '#' {
                        current_number += 1 * ((2 as usize).pow(j as u32));
                    }
                }
                column_numbers.push(current_number);
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
