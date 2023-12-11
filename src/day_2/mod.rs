use crate::{file_reader, traits::Runnable};
use std::collections::HashMap;

pub struct Day2;

fn extract_game_number(parts: &Vec<&str>) -> i32 {
    let game_string = parts[0];
    let substring = game_string[5..].to_string();

    substring.parse().unwrap()
}
//let lines = file_reader::read_lines("src/day_2/input.txt").unwrap();

fn extract_sets(parts: &Vec<&str>) -> Vec<HashMap<String, i32>> {
    let sets_string = parts[1];
    let raw_sets: Vec<&str> = sets_string.split(";").collect();
    let mut sets: Vec<HashMap<String, i32>> = Vec::new();
    for raw_set in raw_sets {
        let raw_set = raw_set.trim();
        let raw_set_parts: Vec<&str> = raw_set.split(", ").collect();
        let mut set_map: HashMap<String, i32> = HashMap::new();

        for raw_set_part in raw_set_parts {
            let num_and_color: Vec<&str> = raw_set_part.split(" ").collect();
            let key = num_and_color[1].to_string();
            let value: i32 = num_and_color[0].parse().unwrap();
            let added_value = set_map.get(&key).unwrap_or(&0) + value;
            set_map.insert(key, added_value);
        }
        sets.push(set_map);
    }

    sets
}

fn extract_sets_raw(parts: &Vec<&str>) -> HashMap<String, i32> {
    let sets_string = parts[1];
    let raw_sets: Vec<&str> = sets_string.split(";").collect();
    let mut set_map: HashMap<String, i32> = HashMap::new();

    for raw_set in raw_sets {
        let raw_set = raw_set.trim();
        let raw_set_parts: Vec<&str> = raw_set.split(", ").collect();

        for raw_set_part in raw_set_parts.clone() {
            let num_and_color: Vec<&str> = raw_set_part.split(" ").collect();
            let key = num_and_color[1].to_string();
            let value: i32 = num_and_color[0].parse().unwrap();

            let current_value = set_map.get(&key).unwrap_or(&0).clone();
            
            if value > current_value {
                set_map.insert(key, value);
            }
        }        
    }
    
    set_map

}


fn part_1(lines: &Vec<String>) {
    let allowed_colors: HashMap<&str, i32> = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect();
    let mut sum = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        let game_number = extract_game_number(&parts);
        let sets: Vec<HashMap<String, i32>> = extract_sets(&parts);

        let passed = sets.iter().all(|set| {
            set.iter().all(|(key, value)| {
                let allowed_value = allowed_colors.get(key.as_str()).unwrap_or(&0);
                value <= allowed_value
            })
        });
        if passed {
            sum += game_number;
        }
    }

    println!("Sum (part1): {}", sum);
}

fn part_2(lines: &Vec<String>) {
    let mut sum = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(":").collect();
        let raw_sets = extract_sets_raw(&parts);
        sum += raw_sets.values().into_iter().fold(1, |acc, &val| acc * val);
    }
    println!("Sum (part2): {}", sum);
}

impl Runnable for Day2 {
    fn run(&self) -> Result<(), std::io::Error> {
        let lines = file_reader::read_lines("src/day_2/input.txt").unwrap();
        part_1(&lines);
        part_2(&lines);
        Ok(())
    }
}
