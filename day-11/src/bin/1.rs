use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut board: Vec<Vec<char>> = vec![];
    //hashmap of number and coordinate
    let mut galaxy_coordinates: HashMap<usize, (usize, usize)> = HashMap::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                board.push(row.chars().collect::<Vec<char>>());
            }
        }
    };
    println!("rows: {}", board.len());
    println!("columns: {}", board[0].len());
    let mut rows_to_double: Vec<usize> = vec![];
    let mut columns_to_double: Vec<usize> = vec![];
    for (i, row) in board.iter().enumerate() {
        let count_galaxies_in_row = row.iter().filter(|&c| *c == '#').count();
        if count_galaxies_in_row == 0 {
            rows_to_double.push(i);
        }
    }
    for i in 0..board[0].len() {
        let count_galaxies_in_column = board.iter().filter(|&row| row[i] == '#').count();
        if count_galaxies_in_column == 0 {
            columns_to_double.push(i);
        }
    }
    while rows_to_double.len() > 0 {
        let row = rows_to_double.pop().unwrap();
        board.insert(row, board[row].clone());
    }
    while columns_to_double.len() > 0 {
        let column_index = columns_to_double.pop().unwrap();
        for i in 0..board.len() {
            board[i].insert(column_index, '.');
        }
    }
    println!("rows: {}", board.len());
    println!("columns: {}", board[0].len());

    let mut count_galaxy = 0;
    for (i, row) in board.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                galaxy_coordinates.insert(count_galaxy, (i, j));
                count_galaxy += 1;
            }
        }
    }

    let mut shortest_paths: HashMap<(usize, usize), usize> = HashMap::new();
    let mut sum_shortest_paths = 0;
    for galaxy_index in 0..count_galaxy {
        let galaxy_coordinate = galaxy_coordinates.get(&galaxy_index).unwrap();
        for second_galaxy_index in 0..count_galaxy {
            let second_galaxy_coordinate = galaxy_coordinates.get(&second_galaxy_index).unwrap();
            if galaxy_index != second_galaxy_index {
                let distance = (galaxy_coordinate.0 as i32 - second_galaxy_coordinate.0 as i32)
                    .abs()
                    + (galaxy_coordinate.1 as i32 - second_galaxy_coordinate.1 as i32).abs();
                if !(shortest_paths.contains_key(&(galaxy_index, second_galaxy_index))
                    || shortest_paths.contains_key(&(second_galaxy_index, galaxy_index)))
                {
                    sum_shortest_paths += distance;
                    shortest_paths.insert((galaxy_index, second_galaxy_index), distance as usize);
                }
            }
        }
    }
    println!("sum_shortest_paths: {}", sum_shortest_paths);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
