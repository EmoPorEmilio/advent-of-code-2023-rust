use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Hand {
    cards: String,
    bid: usize,
    kind: u8,
}

impl Eq for Hand {}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        compare_hands(&self, &other) == Ordering::Equal
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(compare_hands(&self, &other))
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        compare_hands(&self, &other)
    }
}

impl Hand {
    pub fn new(cards: String, bid: usize, kind: u8) -> Self {
        Hand { cards, bid, kind }
    }
}

fn compare_values(a: char, b: char) -> Ordering {
    if get_value(a) > get_value(b) {
        Ordering::Greater
    } else if get_value(a) < get_value(b) {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn get_value(char: char) -> isize {
    if char.is_numeric() {
        char.to_digit(10).unwrap() as isize
    } else {
        if char == 'T' {
            10
        } else if char == 'J' {
            1
        } else if char == 'Q' {
            12
        } else if char == 'K' {
            13
        } else if char == 'A' {
            14
        } else {
            println!("raro");
            0
        }
    }
}

//0 = top card
//1 = one pair
//2 = two pair
//3 = three of a kind
//4 = full house
//5 = four of a kind
//6 = five of a kind
fn calculate_kind_hand(cards: &str) -> u8 {
    //create hashmap of character
    let mut map: std::collections::HashMap<char, i8> = std::collections::HashMap::new();
    let mut amount_of_j = 0;
    for char in cards.chars() {
        if char == 'J' {
            amount_of_j += 1;
        } else {
            let count = map.get(&char);
            if let Some(prev_count) = count {
                map.insert(char, prev_count + 1);
            } else {
                map.insert(char, 1);
            }
        }
    }
    //println!("{}:{:?} y j:{}", cards, map, amount_of_j);
    if map.len() == 0 {
        return 6;
    } else if map.len() == 1 {
        return 6;
    } else if map.len() == 2 {
        if amount_of_j == 3 {
            return 5;
        } else if amount_of_j == 2 {
            return 5;
        } else if amount_of_j == 1 {
            for (_, value) in map.iter() {
                if *value == 3 {
                    return 5;
                }
            }
            return 4;
        } else {
            for (_, value) in map.iter() {
                if *value == 4 {
                    return 5;
                }
            }
            return 4;
        }
    } else if map.len() == 3 {
        if amount_of_j >= 1 {
            return 3;
        } else {
            for (_, value) in map.iter() {
                if *value == 3 {
                    return 3;
                }
                if *value == 2 {
                    return 2;
                }
            }
        }
    } else if map.len() == 4 {
        return 1;
    } else if map.len() == 5 {
        return 0;
    }
    println!("raro");
    return 100;
}

fn compare_hands<'a, 'b>(a: &'a Hand, b: &'b Hand) -> Ordering {
    if a.kind < b.kind {
        return Ordering::Greater;
    } else if a.kind > b.kind {
        return Ordering::Less;
    } else {
        for (i, cards) in a.cards.chars().enumerate() {
            let option_b_char = b.cards.chars().nth(i);
            if let Some(b_char) = option_b_char {
                if compare_values(cards, b_char) == Ordering::Less {
                    return Ordering::Greater;
                } else if compare_values(cards, b_char) == Ordering::Greater {
                    return Ordering::Less;
                }
            }
        }
    }
    return Ordering::Equal;
}

fn main() {
    let mut heap: BinaryHeap<Hand> = BinaryHeap::new();
    if let Ok(lines) = read_lines("./input.txt") {
        // vector of integers
        for line in lines {
            if let Ok(row) = line {
                let strings = row.split_whitespace().collect::<Vec<&str>>();
                heap.push(Hand::new(
                    strings[0].to_string(),
                    strings[1].parse::<usize>().unwrap(),
                    calculate_kind_hand(strings[0]),
                ));
            }
        }
    };
    let mut rank = 0;
    let mut multiplier = 0;
    while let Some(hand) = heap.pop() {
        rank += 1;
        multiplier += rank * hand.bid;
        println!("{} {} {}", rank, hand.cards, hand.kind)
    }
    println!("The answer is {}", multiplier);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
