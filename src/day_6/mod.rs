use crate::{file_reader, traits::Runnable};

pub struct Day6;
fn part_1(lines: &Vec<String>) {
    let time_stamps: Vec<i64> = lines[0]
    .split_whitespace()
    .filter_map(|s| s.parse().ok())
    .collect();

    let distances: Vec<i64> = lines[1]
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let mut totals: Vec<i64> = Vec::new();

    for i in 0..time_stamps.len() {
        let time = time_stamps[i];
        let distance = distances[i];
        totals.push(0);
        let mut previous;
        let mut sum = 0;
        for j in 0..=time {
            previous = sum;
            sum = j * (time - j);
            if j != 0 && previous == sum {
                totals[i] *= 2;
                break;
            }

            if sum > distance {
                totals[i as usize] += 1;
            }
        }

    }

    println!("Sum is: {:?}", totals.iter().fold(1, |acc, &x| acc * x))

}

impl Runnable for Day6 {
    fn run(&self) -> Result<(), std::io::Error> {
        let lines = file_reader::read_lines("src/day_6/input.txt").unwrap();
        part_1(&lines);
        // Part 2 is the same as part 1
        
        Ok(())
    }
}
