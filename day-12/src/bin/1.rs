use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let row_and_values: Vec<(String, Vec<usize>)> = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        for (i, line) in lines.enumerate() {
            if let Ok(row) = line {
                let mut new_element: (String, Vec<usize>) = (String::from(""), vec![]);
                new_element.1 = row.split_whitespace().collect::<Vec<&str>>()[1]
                    .split(',')
                    .flat_map(|x| x.parse::<usize>())
                    .collect::<Vec<usize>>();
                new_element.0 = row.split_whitespace().collect::<Vec<&str>>()[0].to_string();
                row_and_values.push(new_element);
                println!("{:?}", new_element);
            }
        }
    };
    let mut row_number = 0;
    let mut count_valid = 0;
    let len = row_and_values.len();
    let current_count = 0;
    while row_number < len {
        let row = row_and_values[row_number];
        for char in row.0.chars() {}
        row_number += 1;
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
