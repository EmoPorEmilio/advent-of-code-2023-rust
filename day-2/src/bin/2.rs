use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum_powers = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(game) = line {
                let split_game = game.split(':').collect::<Vec<&str>>();
                let game_id = split_game[0].split(' ').collect::<Vec<&str>>()[1];
                let game_result = split_game[1];
                let sets = game_result.split(';').collect::<Vec<&str>>();
                let mut min_red = 0;
                let mut min_green = 0;
                let mut min_blue = 0;
                for set in sets {
                    let balls = set.split(',').collect::<Vec<&str>>();
                    for ball in balls {
                        let color_numbers = ball.trim().split(' ').collect::<Vec<&str>>();
                        match color_numbers[1] {
                            "red" => {
                                if color_numbers[0].parse::<i32>().unwrap() > min_red {
                                    min_red = color_numbers[0].parse::<i32>().unwrap();
                                }
                            }
                            "green" => {
                                if color_numbers[0].parse::<i32>().unwrap() > min_green {
                                    min_green = color_numbers[0].parse::<i32>().unwrap();
                                }
                            }
                            "blue" => {
                                if color_numbers[0].parse::<i32>().unwrap() > min_blue {
                                    min_blue = color_numbers[0].parse::<i32>().unwrap();
                                }
                            }
                            _ => {}
                        }
                    }
                }

                sum_powers += min_red * min_green * min_blue;
            }
        }
        println!("{}", sum_powers);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
