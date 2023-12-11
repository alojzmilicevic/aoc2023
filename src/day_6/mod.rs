use crate::{file_reader, traits::Runnable};

pub struct Day6;

impl Runnable for Day6 {
    fn run(&self) -> Result<(), std::io::Error> {
        let lines = file_reader::read_lines("src/day_6/input.txt").unwrap();
        println!("Day 6: {:?}", lines);
        Ok(())
    }
}
