pub struct Day3;
use std::{collections::HashMap, usize};

use crate::{file_reader, traits::Runnable};

fn is_valid_position(row: i32, col: i32, n: i32, m: i32) -> bool {
    row < n && row >= 0 && col < m && col >= 0
}

fn to_one_dimension(row: i32, col: i32, m: i32) -> usize {
    (row * m + col) as usize
}

fn get_adjacent(row: i32, start: i32, end: i32, lines: &Vec<String>) -> Vec<char> {
    let mut adjacent: Vec<char> = Vec::new();

    for i in row - 1..=row + 1 {
        for j in start - 1..=end + 1 {
            if is_valid_position(i, j, lines.len() as i32, lines[0].len() as i32) {
                let line = &lines[i as usize];
                adjacent.push(line.chars().nth(j as usize).unwrap());
            }
        }
    }

    adjacent
}

fn get_adjacent_star(row: i32, start: i32, end: i32, lines: &Vec<String>) -> i32 {
    let mut star_pos = -1;

    for i in row - 1..=row + 1 {
        for j in start - 1..=end + 1 {
            if is_valid_position(i, j, lines.len() as i32, lines[0].len() as i32) {
                let line = &lines[i as usize];
                let c = &line.chars().nth(j as usize).unwrap();

                if c == &'*' {
                    star_pos = to_one_dimension(i, j, lines[0].len() as i32) as i32;
                    break;
                }
            }
        }
    }

    star_pos
}

fn part_1(lines: &Vec<String>) {
    let mut sum = 0;
    let allowed_chars: [char; 11] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '.'];

    for (row, line) in lines.iter().enumerate() {
        let mut start: i32 = -1;
        let mut end;

        for (col, c) in line.chars().enumerate() {
            let is_digit = c.is_digit(10);
            let last_col = col == line.len() - 1;

            if is_digit && start == -1 {
                start = col as i32;
            }

            if (!is_digit || last_col) && start != -1 {
                if is_digit {
                    end = col as i32;
                } else {
                    end = col as i32 - 1;
                }

                let adjacent: Vec<char> = get_adjacent(row as i32, start, end, lines);

                for c in adjacent {
                    if !allowed_chars.contains(&c) {
                        let value = line[start as usize..=end as usize].parse::<i32>().unwrap();

                        sum += value;
                        break;
                    }
                }

                start = -1;
            }
        }
    }

    println!("Sum (part1): {}", sum);
}

fn part_2(lines: &Vec<String>) {
    let mut map: HashMap<i32, Vec<i32>> = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        let mut start: i32 = -1;
        let mut end;

        for (col, c) in line.chars().enumerate() {
            let is_digit = c.is_digit(10);
            let last_col = col == line.len() - 1;

            if is_digit && start == -1 {
                start = col as i32;
            }

            if (!is_digit || last_col) && start != -1 {
                if is_digit {
                    end = col as i32;
                } else {
                    end = col as i32 - 1;
                }

                let star_pos = get_adjacent_star(row as i32, start, end, lines);

                if star_pos != -1 {
                    let value: i32 = line[start as usize..=end as usize].parse().unwrap();

                    map.entry(star_pos).or_insert(Vec::new()).push(value);
                }

                start = -1;
            }
        }
    }
    let mut sum = 0;

    map.values()
        .filter(|values| values.len() == 2)
        .for_each(|values| sum += values[0] * values[1]);

    println!("Sum (part2): {}", sum);
}

impl Runnable for Day3 {
    fn run(&self) -> Result<(), std::io::Error> {
        let lines = file_reader::read_lines("src/day_3/input.txt").unwrap();

        part_1(&lines);
        part_2(&lines);
        Ok(())
    }
}
