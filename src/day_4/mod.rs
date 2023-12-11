use std::collections::HashMap;

use crate::{file_reader, traits::Runnable};

fn extract_left_and_right_numbers(line: &str) -> (Vec<i32>, Vec<i32>) {
    // Splitting the line into words
    // Split the line at the '|' character
    let parts: Vec<&str> = line.split('|').collect();

    let left_numbers: Vec<i32> = parts[0]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let right_numbers: Vec<i32> = parts[1]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    (left_numbers, right_numbers)
}
fn part_1(lines: &Vec<String>) {
    let mut sum = 0;

    for line in lines {
        let (left_numbers, right_numbers) = extract_left_and_right_numbers(line);

        let mut total_hits = 0;
        for number in right_numbers.iter() {
            if left_numbers.contains(number) {
                total_hits += 1;
            }
        }

        if total_hits != 0 {
            let base: i32 = 2;
            sum += base.pow(total_hits - 1);
        }
    }

    println!("Sum (part1): {}", sum);
}

fn part_2(lines: &Vec<String>) {
    let mut memo_table: HashMap<i32, i32> = HashMap::new();

    let reversed_lines: Vec<&String> = lines.iter().rev().collect();
    for (index, line) in reversed_lines.iter().enumerate() {
        let (left_numbers, right_numbers) = extract_left_and_right_numbers(line);

        let mut total_hits = 0;
        for number in right_numbers.iter() {
            if left_numbers.contains(number) {
                total_hits += 1;
            }
        }

        if total_hits == 0 {
            memo_table.insert((lines.len() - index) as i32, 1);
        } else {
            let start = lines.len() - index;
            let end = start + total_hits;
            let mut sum = 1;
            for i in start + 1..=end {
                sum += memo_table.get(&(i as i32)).unwrap();
            }

            memo_table.insert((lines.len() - index) as i32, sum);
        }
    }

    println!("Sum (part2): {}", memo_table.values().sum::<i32>());
}

pub struct Day4;
impl Runnable for Day4 {
    fn run(&self) -> Result<(), std::io::Error> {
        let lines = file_reader::read_lines("src/day_4/input.txt").unwrap();

        part_1(&lines);
        part_2(&lines);
        Ok(())
    }
}
