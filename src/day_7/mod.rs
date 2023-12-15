use crate::{file_reader, traits::Runnable};
use std::collections::HashMap;

const CARDS: [char; 13] = [
    'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
];

const CARD_VALUES: [i32; 13] = [
    14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 1,
];

#[derive(Debug)]
#[derive(PartialEq, PartialOrd)]
enum HandTypes {
    None,
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn determine_hand_type(hand_string: &str) -> HandTypes {
    let mut char_map: HashMap<char, i32> = HashMap::new();
    for c in hand_string.chars() {
        *char_map.entry(c).or_insert(0) += 1;
    }

    let mut frequencies: Vec<i32> = char_map.values().cloned().collect();
    frequencies.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order

    return match frequencies.as_slice() {
        [5, ..] => HandTypes::FiveOfAKind,
        [4, 1, ..] => HandTypes::FourOfAKind,
        [3, 2, ..] => HandTypes::FullHouse,
        [3, 1, 1, ..] => HandTypes::ThreeOfAKind,
        [2, 2, 1, ..] => HandTypes::TwoPair,
        [2, 1, 1, 1, ..] => HandTypes::OnePair,
        [1, 1, 1, 1, 1, ..] => HandTypes::HighCard,
        _ => HandTypes::None,
    };
}

fn compare_hands(a: &str, b: &str) -> bool {
    let a_hand_type = determine_hand_type(a);
    let b_hand_type = determine_hand_type(b);

    if a_hand_type > b_hand_type {
        return true;
    } else if a_hand_type < b_hand_type {
        return false;
    } else {
        for (a, b) in a.chars().zip(b.chars()) {
            let a_index = CARDS.iter().position(|&c| c == a).unwrap();
            let b_index = CARDS.iter().position(|&c| c == b).unwrap();

            if CARD_VALUES[a_index] > CARD_VALUES[b_index] {
                return true;
            } else if CARD_VALUES[a_index] < CARD_VALUES[b_index] {
                return false;
            }
        }
    }

    return true;
}

fn get_parts(line: &str) -> (&str, &str) {
    let mut parts = line.split_whitespace();

    let hand_string: &str = parts.next().unwrap();
    let bet = parts.next().unwrap();

    return (hand_string, bet);
}

pub struct Day7;
fn part_1(lines: &Vec<String>) {
    let mut sorted_values: Vec<(&str, &str)> = Vec::new();
    
    for (i, line) in lines.iter().enumerate() {
        let line_parts = get_parts(line);

        if i == 0 {
            sorted_values.push(line_parts);
            continue;
        }

        for (index, value) in sorted_values.iter().enumerate() {
            if compare_hands(line_parts.0, value.0) {
                sorted_values.insert(index, line_parts);
                break;
            } else if index == sorted_values.len() - 1 {
                sorted_values.push(line_parts);
                break;
            }
        }
    }

    println!("Sorted values: {:?}", sorted_values);
    
    let mut sum = 0;
    for (index, pair) in sorted_values.iter().enumerate().rev() {
        sum += (index + 1) as i64 * pair.1.parse::<i64>().unwrap();
    }

    println!("Sum: {}", sum);
}

impl Runnable for Day7 {
    fn run(&self) -> Result<(), std::io::Error> {
        let lines = file_reader::read_lines("src/day_7/input.txt").unwrap();
        part_1(&lines);

        Ok(())
    }
}
