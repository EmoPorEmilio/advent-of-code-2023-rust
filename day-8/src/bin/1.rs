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
    let mut current_key = String::from("AAA");
    while let Some(char) = order.get(step) {
        if current_key == String::from("ZZZ") {
            println!("{}", current_key);
            break;
        }
        println!("{}", current_key);
        count += 1;
        if *char == 'L' {
            current_key = map.get(&current_key).unwrap().0.to_string();
        } else {
            current_key = map.get(&current_key).unwrap().1.to_string();
        }
        step += 1;
        if order.get(step) == None {
            step = 0;
        }
    }
    println!("{}", count);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
