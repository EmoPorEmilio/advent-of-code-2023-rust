use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Map_Seeds = (isize, isize, isize);
fn main() {
    let mut seed_maps: Vec<HashMap<isize, isize>> = Vec::new();
    let mut maps: Vec<Vec<Map_Seeds>> = Vec::new();
    let mut seeds: Vec<isize> = Vec::new();
    let mut current_map: HashMap<isize, isize> = HashMap::new();
    let mut min_location = isize::max_value();
    let mut reading_map = false;
    let mut map_being_processed: Vec<Map_Seeds> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        // vector of integers
        for (i, line) in lines.enumerate() {
            if let Ok(row) = line {
                if i == 0 {
                    let seeds_string = row.split(":").collect::<Vec<&str>>()[1];
                    for seed in seeds_string.split_whitespace() {
                        seeds.push(seed.parse::<isize>().unwrap());
                    }
                } else if row.is_empty() {
                    if reading_map {
                        maps.push(map_being_processed.clone());
                        map_being_processed = Vec::new();
                    }
                } else if row.contains("map") {
                    reading_map = true;
                } else {
                    let numbers = row.split(" ").collect::<Vec<&str>>();
                    let destination_start = numbers[0].parse::<isize>().unwrap();
                    let source_start = numbers[1].parse::<isize>().unwrap();
                    let length = numbers[2].parse::<isize>().unwrap();
                    map_being_processed.push((destination_start, source_start, length));
                }
            }
        }
        maps.push(map_being_processed.clone());

        for (index, map) in maps.iter().enumerate() {
            min_location = isize::max_value();
            let mut seed_map = HashMap::new();
            for seed in &seeds {
                let mut corresponding_value_seed = seed;
                for i in 0..index {
                    if seed_maps[i].contains_key(corresponding_value_seed) {
                        corresponding_value_seed =
                            seed_maps[i].get(corresponding_value_seed).unwrap();
                    }
                }
                for numbers in map {
                    let destination_start = numbers.0;
                    let source_start = numbers.1;
                    let length = numbers.2;
                    if (source_start..(source_start + length)).contains(corresponding_value_seed) {
                        let value_to_insert =
                            destination_start + (corresponding_value_seed - source_start);
                        seed_map.insert(*corresponding_value_seed, value_to_insert);
                        if value_to_insert < min_location {
                            min_location = value_to_insert;
                        }
                    }
                }
                if !seed_map.contains_key(corresponding_value_seed) {
                    seed_map.insert(*corresponding_value_seed, *corresponding_value_seed);
                    if min_location > *corresponding_value_seed {
                        min_location = *corresponding_value_seed;
                    }
                }
            }
            seed_maps.push(seed_map.clone());
        }
        println!("{}", min_location)
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
