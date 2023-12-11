use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut histories: Vec<Vec<isize>> = vec![];
    if let Ok(lines) = read_lines("./input.txt") {
        for (i, line) in lines.enumerate() {
            if let Ok(row) = line {}
        }
    };
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
