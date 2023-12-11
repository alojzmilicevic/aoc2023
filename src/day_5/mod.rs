use crate::{file_reader, traits::Runnable};

fn extract_numbers(line: &str) -> Vec<i64> {
    // Splitting the line into words
    // Split the line at the '|' character
    line.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn extract_seeds(line: &str) -> Vec<(i64, i64)> {
    // Splitting the line into words
    // Split the line at the '|' character
    let raw_seeds: Vec<i64> = line.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    return raw_seeds.chunks(2).map(|x| (x[0], x[1])).collect();
}

fn generate_map(lines: &Vec<String>) -> Vec<Vec<Vec<i64>>> {
    let mut seed_map: Vec<Vec<Vec<i64>>> = Vec::new();

    let mut start: usize = 0;
    for (index, line) in lines.iter().enumerate() {
        if index == 0 {
            continue;
        }

        if line.contains(":") && start == 0 {
            start = index + 1;
        }

        if (line == "" || index == lines.len() - 1) && start != 0 {
            seed_map.push(Vec::new());
            for i in start..index {
                let numbers: Vec<i64> = extract_numbers(&lines[i]);
                let idx = seed_map.len() - 1;
                seed_map[idx].push(numbers);
            }
            start = 0;
        }
    }

    seed_map
}

fn part_1(lines: &Vec<String>, seeds: &Vec<i64>) {
    //let mut sum = 0;
    let map_data = &generate_map(lines);
    let mut res: Vec<i64> = Vec::new();
    for seed in seeds.iter() {
        let mut target = *seed;

        for map_section in map_data {
            for instruction in map_section.iter() {
                let destination_start = instruction[0];
                let source_start = instruction[1];
                let length = instruction[2];

                // TARGET is correct for fert to water and water to light
                if target >= source_start && target < (source_start + length) {
                    target = target + (destination_start - source_start);
                    break;
                }
            }
        }
        res.push(target);
    }

    println!("Result (part1): {:?}", res.iter().min().unwrap());
}

pub struct Day5;
impl Runnable for Day5 {
    fn run(&self) -> Result<(), std::io::Error> {
        let lines = file_reader::read_lines("src/day_5/input.txt").unwrap();
        let raw_seeds: Vec<i64> = extract_numbers(&lines[0]);
        let raw_seeds_p2 = extract_seeds(&lines[0]);
        let mut seeds_part_2: Vec<i64> = Vec::new();
        part_1(&lines, &raw_seeds);

        for seed_data in raw_seeds_p2.iter() {
            let start = seed_data.0;
            let end = seed_data.1;
            for i in start..start + end {
                seeds_part_2.push(i);
            }
        }
        println!("Seeds:");

        part_1(&lines, &seeds_part_2);
        Ok(())
    }
}
