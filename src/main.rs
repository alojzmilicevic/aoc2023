mod day_1;
mod day_10;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod file_reader;
mod traits;

use crate::{
    day_1::Day1, day_10::Day10, day_2::Day2, day_3::Day3, day_4::Day4, day_5::Day5, day_6::Day6,
    day_7::Day7, day_8::Day8, day_9::Day9, traits::Runnable,
};

fn main() {
    let days: Vec<Box<dyn Runnable>> = vec![
        Box::new(Day1),
        Box::new(Day2),
        Box::new(Day3),
        Box::new(Day4),
        Box::new(Day5),
        Box::new(Day6),
        Box::new(Day7),
        Box::new(Day8),
        Box::new(Day9),
        Box::new(Day10),
    ];

    days[7 - 1].run().unwrap();

    /*
    for (index, day) in days.iter().enumerate() {
        println!("Running day {}", index + 1);
        if let Err(e) = day.run() {
            println!("An error occurred on Day: {} - {}", index, e);
        }
    }
     */
}
