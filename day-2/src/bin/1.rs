use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    let mut sum_ids = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(game) = line {
                let split_game = game.split(':').collect::<Vec<&str>>();
                let game_id = split_game[0].split(' ').collect::<Vec<&str>>()[1];
                let game_result = split_game[1];
                let sets = game_result.split(';').collect::<Vec<&str>>();
                let mut valid_game = true;
                for set in sets {
                    let balls = set.split(',').collect::<Vec<&str>>();
                    for ball in balls {
                        let color_numbers = ball.trim().split(' ').collect::<Vec<&str>>();
                        match color_numbers[1] {
                            "red" => {
                                if color_numbers[0].parse::<i32>().unwrap() > max_red {
                                    valid_game = false;
                                }
                            }
                            "green" => {
                                if color_numbers[0].parse::<i32>().unwrap() > max_green {
                                    valid_game = false;
                                }
                            }
                            "blue" => {
                                if color_numbers[0].parse::<i32>().unwrap() > max_blue {
                                    valid_game = false;
                                }
                            }
                            _ => {}
                        }
                    }
                }

                if valid_game {
                    sum_ids += game_id.parse::<i32>().unwrap();
                }
            }
        }
        println!("{}", sum_ids);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
