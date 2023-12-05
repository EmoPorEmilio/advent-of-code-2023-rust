use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut seed_maps: Vec<HashMap<isize, isize>> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        // vector of integers
        let mut seeds: Vec<isize> = Vec::new();
        let mut current_map: HashMap<isize, isize> = HashMap::new();
        let mut min_location = isize::max_value();
        for (i, line) in lines.enumerate() {
            if let Ok(row) = line {
                if i == 0 {
                    let seeds_string = row.split(":").collect::<Vec<&str>>()[1];
                    for seed in seeds_string.split_whitespace() {
                        seeds.push(seed.parse::<isize>().unwrap());
                    }
                }
                if !row.is_empty() {
                    if row.contains("map") {
                        if current_map.len() > 0 {
                            seed_maps.push(current_map.clone());
                            current_map = HashMap::new();
                        }
                        min_location = isize::max_value();
                    }
                    if row.chars().collect::<Vec<char>>()[0].is_numeric() {
                        let numbers = row.split(" ").collect::<Vec<&str>>();
                        let destination_start = numbers[0].parse::<isize>().unwrap();
                        let source_start = numbers[1].parse::<isize>().unwrap();
                        let length = numbers[2].parse::<isize>().unwrap();
                        for seed in &seeds {
                            let mut corresponding_value_seed = seed;
                            let mut seed_map_index = 0;
                            while seed_maps.len() >= seed_map_index + 1
                                && seed_maps[seed_map_index].len() > 0
                                && seed_maps[seed_map_index].contains_key(corresponding_value_seed)
                            {
                                corresponding_value_seed = seed_maps[seed_map_index]
                                    .get(corresponding_value_seed)
                                    .unwrap();
                                seed_map_index += 1;
                            }
                            // implement check *corresponding_value_seed is in range of source_start..(source_start + length -1)
                            if (source_start..(source_start + length - 1))
                                .contains(corresponding_value_seed)
                            {
                                let value_to_insert =
                                    destination_start + (corresponding_value_seed - source_start);
                                current_map.insert(*corresponding_value_seed, value_to_insert);
                                if value_to_insert < min_location {
                                    min_location = value_to_insert;
                                }
                            }
                        }
                    }
                }
            }
        }

        seed_maps.push(current_map.clone());
        for (key, value) in &seed_maps[1] {
            println!("{}: {}", key, value);
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
