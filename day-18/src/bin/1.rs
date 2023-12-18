use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut coordinates: HashMap<isize, BinaryHeap<isize>> = HashMap::new();
    coordinates.insert(0, BinaryHeap::from([0]));
    let mut current_coordinate = (0, 0);
    let mut min_x = 100000;
    let mut max_x = 0;
    let mut min_y = 100000;
    let mut max_y = 0;
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                let mut current_direction = '?';
                for (i, component) in row.split_whitespace().enumerate() {
                    if i == 0 {
                        current_direction = component.chars().nth(0).unwrap();
                    } else if i == 1 {
                        let steps = component.parse::<isize>().unwrap();
                        for i in 1..steps {
                            if current_direction == 'L' {
                            } else if current_direction == 'R' {
                                current_coordinate.0 += component.parse::<isize>().unwrap();
                            } else if current_direction == 'U' {
                                insert_to_heap(
                                    &coordinates.get(&current_coordinate.0).unwrap().clone(),
                                    current_coordinate.1 - i,
                                );
                            } else if current_direction == 'D' {
                                insert_to_heap(
                                    &coordinates.get(&current_coordinate.0).unwrap().clone(),
                                    current_coordinate.1 + i,
                                );
                            }
                        }

                        if current_coordinate.0 <= min_x {
                            min_x = current_coordinate.0;
                        }
                        if current_coordinate.0 >= max_x {
                            max_x = current_coordinate.0;
                        }
                        if current_coordinate.1 <= min_y {
                            min_y = current_coordinate.1;
                        }
                        if current_coordinate.1 >= max_y {
                            max_y = current_coordinate.1;
                        }
                        if let Some(coordinates_y) = coordinates.get(&current_coordinate.0) {
                            let mut new_vec = coordinates_y.clone();
                            new_vec.push(current_coordinate.1);
                            coordinates.insert(current_coordinate.0, new_vec);
                        } else {
                            coordinates.insert(current_coordinate.0, vec![current_coordinate.1]);
                        }
                    }
                }
            }
        }
    };
    let mut lava_pieces = 0;
    //print hashmap coordinates
    for (x, y) in coordinates.clone() {
        println!("{} {:?}", x, y);
    }
    println!("{} {} {} {}", min_x, max_x, min_y, max_y);
    for i in min_x..max_x {
        for j in min_y..max_y {
            if coordinates.contains_key(&i) {
                if let Some(coordinate_y) = coordinates.get(&i) {
                    let mut found = false;
                    let mut before = 0;
                    let mut after = 0;
                    for y in coordinate_y {
                        if *y == j {
                            found = true;
                            println!("{} {} {} {}", i, j, before, after)
                        } else if *y < j {
                            before += 1;
                        }
                    }
                    if found {
                        lava_pieces += 1;
                    }
                }
            }
        }
    }
    println!("{}", lava_pieces);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
