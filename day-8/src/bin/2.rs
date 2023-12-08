use num::integer::lcm;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Instruction = (String, String);
fn main() {
    let mut value_1;
    let mut value_2;
    let mut left_right_order = String::new();
    let mut map: HashMap<String, Instruction> = HashMap::new();
    if let Ok(lines) = read_lines("./input.txt") {
        // vector of integers
        for (i, line) in lines.enumerate() {
            if let Ok(row) = line {
                if i == 0 {
                    left_right_order = row.to_string();
                } else if row != "" {
                    //1 es antes del =
                    //2 le falta sacarle el ( y la coma al final
                    //3 le falta sacarle el parentesis al final
                    let strings_slots = row.split_whitespace().collect::<Vec<&str>>();
                    let key = strings_slots[0];
                    value_1 = strings_slots[2].split_at(strings_slots[2].len() - 1).0;
                    value_1 = value_1.split_at(1).1;

                    value_2 = strings_slots[3].split_at(strings_slots[3].len() - 1).0;
                    map.insert(key.to_string(), (value_1.to_string(), value_2.to_string()));
                }
            }
        }
    };
    let mut count = 0;
    let order = left_right_order.chars().collect::<Vec<char>>();
    let mut step = 0;
    let mut current_values: Vec<String> = Vec::new();
    for (key, _) in map.iter() {
        if key.chars().nth(2).unwrap() == 'A' {
            current_values.push(key.to_string());
        }
    }
    let mut finished = false;
    let mut first_count_for_values: HashMap<usize, usize> = HashMap::new();
    while !finished {
        count += 1;
        let char_direction = order.get(step).unwrap();
        //let mut new_values = Vec::new();
        for i in 0..current_values.len() {
            if !first_count_for_values.contains_key(&i) {
                let first_current_value = current_values[i].to_string();
                let new_value;
                if *char_direction == 'L' {
                    new_value = map.get(&first_current_value).unwrap().0.to_string();
                } else {
                    new_value = map.get(&first_current_value).unwrap().1.to_string();
                }
                if new_value.chars().nth(2).unwrap() == 'Z' {
                    first_count_for_values.insert(i, count);
                }
                current_values[i] = new_value;
            }
        }
        if first_count_for_values.len() == current_values.len() {
            finished = true;
        }
        step += 1;
        if (step) == order.len() {
            step = 0;
        }
    }
    //calculate LCM between all the values in the hashmap
    let mut lcm = 1;
    for (_, value) in first_count_for_values.iter() {
        lcm = num::integer::lcm(lcm, *value);
    }
    println!("{}", lcm);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
