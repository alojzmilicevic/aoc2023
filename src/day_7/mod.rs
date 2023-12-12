use crate::{file_reader, traits::Runnable};
use std::collections::HashMap;

enum Types {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

pub struct Day7;
fn part_1(lines: &Vec<String>) {
    println!("Lines: {:?}", lines);

    let line = "32T3k";

    let chars = line.chars();
    let mut char_map: HashMap<char, i32> = HashMap::new();
    for c in chars {
        *char_map.entry(c).or_insert(0) += 1;
    }

    let length = char_map.values().len();
    let mut types: Vec<Types> = Vec::new();

    // What x numbers add up to 5?
    if length == 1 {
        types.push(Types::FiveOfAKind);
    } else if length == 2 {
        // Four of a kind + single
        // Full house + pair
    } else if length == 3 {
        // three of a kind + single + single
        // two pair + two pair + single
    } else if length == 4 {
        // Check if it's a one pair
    } else if length == 5 {
        types.push(Types::HighCard);
    }
    println!("Char map: {:#?} - ({})", char_map, char_map.values().len());
}

impl Runnable for Day7 {
    fn run(&self) -> Result<(), std::io::Error> {
        let lines = file_reader::read_lines("src/day_7/input.txt").unwrap();
        part_1(&lines);

        Ok(())
    }
}
