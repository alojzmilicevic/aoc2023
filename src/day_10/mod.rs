use crate::{file_reader, traits::Runnable};

pub struct Day10;
fn part_1(lines: &Vec<String>) {
    println!("Lines: {:?}", lines);
}

impl Runnable for Day10 {
    fn run(&self) -> Result<(), std::io::Error> {
        let lines = file_reader::read_lines("src/day_10/input.txt").unwrap();
        part_1(&lines);
        
        Ok(())
    }
}
