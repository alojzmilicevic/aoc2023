use crate::{file_reader, traits::Runnable};

pub struct Day8;
fn part_1(lines: &Vec<String>) {
    println!("Lines: {:?}", lines);
}

impl Runnable for Day8 {
    fn run(&self) -> Result<(), std::io::Error> {
        let lines = file_reader::read_lines("src/day_8/input.txt").unwrap();
        part_1(&lines);
        
        Ok(())
    }
}
