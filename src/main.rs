mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod file_reader;
mod traits;

use crate::{day_1::Day1, day_2::Day2, day_3::Day3, day_4::Day4, day_5::Day5, day_6::Day6, traits::Runnable};
use std::io::stdin;

fn main() {
    println!("Enter the day number to run: ");

    let mut day_number = String::new();
    stdin()
        .read_line(&mut day_number)
        .expect("Failed to read line");

    match day_number.trim() {
        "1" => {
            let day = Day1;
            if let Err(e) = day.run() {
                println!("An error occurred on Day 1: {}", e);
            }
        }
        "2" => {
            let day = Day2;
            if let Err(e) = day.run() {
                println!("An error occurred on Day 2: {}", e);
            }
        }
        "3" => {
            let day = Day3;
            if let Err(e) = day.run() {
                println!("An error occurred on Day 3: {}", e);
            }
        }
        "4" => {
            let day = Day4;
            if let Err(e) = day.run() {
                println!("An error occurred on Day 4: {}", e);
            }
        }
        "5" => {
            let day = Day5;
            if let Err(e) = day.run() {
                println!("An error occurred on Day 5: {}", e);
            }
        }
        "6" => {
            let day = Day6;
            if let Err(e) = day.run() {
                println!("An error occurred on Day 6: {}", e);
            }
        }
        _ => println!("Day not implemented"),
    }
}
