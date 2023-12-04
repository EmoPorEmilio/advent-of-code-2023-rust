use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sum = 0;
    let mut x_copies_of_nth_game: HashMap<usize, i32> = HashMap::new();
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(row) = line {
                let game_number: usize = row.split(':').collect::<Vec<&str>>()[0]
                    .split_whitespace()
                    .collect::<Vec<&str>>()[1]
                    .parse()
                    .unwrap();
                let numbers = row.split(':').collect::<Vec<&str>>()[1];
                let winning_numbers = numbers.split('|').collect::<Vec<&str>>()[0];
                let own_numbers = numbers.split('|').collect::<Vec<&str>>()[1];
                let mut count_winning = 0;
                x_copies_of_nth_game.entry(game_number).or_insert(1);
                let copies = *x_copies_of_nth_game.get(&game_number).unwrap();
                sum += copies;
                println!(
                    "game_number: {}, copies: {}, x_copies_of_nth_game: {:?}",
                    game_number, copies, x_copies_of_nth_game
                );

                for number in own_numbers.split_whitespace() {
                    for winning_number in winning_numbers.split_whitespace() {
                        if number == winning_number {
                            count_winning += 1;
                        }
                    }
                }
                for i in (game_number + 1)..=game_number + count_winning {
                    x_copies_of_nth_game.entry(i).or_insert(1);
                    let count = *x_copies_of_nth_game.get(&i).unwrap();

                    x_copies_of_nth_game.insert(i, count + copies);
                }
            }
        }
        println!("{}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
