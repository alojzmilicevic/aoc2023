use crate::{file_reader, traits::Runnable};

pub struct Day9;
fn part_1(lines: &Vec<String>) {
    println!("Lines: {:?}", lines);
}

impl Runnable for Day9 {
    fn run(&self) -> Result<(), std::io::Error> {
        let lines = file_reader::read_lines("src/day_9/input.txt").unwrap();
        part_1(&lines);
        
        Ok(())
    }
}
