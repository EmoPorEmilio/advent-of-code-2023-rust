use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type MapSeeds = (isize, isize, isize);
fn main() {
    let mut maps: Vec<Vec<MapSeeds>> = Vec::new();
    let mut seeds: Vec<(isize, isize)> = Vec::new();
    let mut current_map: HashMap<isize, isize> = HashMap::new();
    let mut min_location = isize::max_value();
    let mut reading_map = false;
    let mut map_being_processed: Vec<MapSeeds> = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        // vector of integers
        for (i, line) in lines.enumerate() {
            if let Ok(row) = line {
                if i == 0 {
                    let seeds_string = row.split(":").collect::<Vec<&str>>()[1];
                    let mut first = true;
                    let mut first_seed = 0;
                    for seed in seeds_string.split_whitespace() {
                        if first {
                            first_seed = seed.parse::<isize>().unwrap();
                            first = false;
                        } else {
                            seeds.push((first_seed, first_seed + seed.parse::<isize>().unwrap()));
                            first = true;
                        }
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

        for (index, seed_ranges) in seeds.iter().enumerate() {
            let (first_seed, last_seed) = seed_ranges;
            let mut ranges: Vec<(isize, isize)> = Vec::new();
            ranges.push((*first_seed, *last_seed));
            for map in &maps {
                let mut temporal_ranges: Vec<(isize, isize)> = Vec::new();
                for range in &ranges {
                    let (start_seed, end_seed) = range;
                    for map_seed in map {
                        let (destination_start, source_start, length) = map_seed;
                        if *source_start <= *start_seed && *end_seed <= *source_start + *length {
                            temporal_ranges
                                .push((*destination_start, *destination_start + *length));
                        } else if *source_start > *start_seed && *source_start + *length < *end_seed
                        {
                            /*me falta resolver cómo agregarle al range en rust */
                            ranges.push((*start_seed, *source_start));
                            temporal_ranges.push((
                                *destination_start,
                                *destination_start + *source_start - *start_seed,
                            ));
                        }
                        /*me faltan 2 casos: cuando está en el medio (tres subdivisiones) y cuando está al final (2 subdivisiones) */
                    }
                }
                for range in &ranges {
                    temporal_ranges.push((range.0, range.1));
                }
                ranges = temporal_ranges.clone();
            }
            if index == seeds.len() - 1 {
                for range in ranges {
                    if range.0 < min_location {
                        min_location = range.0;
                    }
                }
            }
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
