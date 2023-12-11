use crate::file_reader;
use crate::traits::Runnable;

fn find_numeric_chars(input: &str) -> Result<i32, &'static str> {
    let mut front_iter = input.chars();
    let mut back_iter = input.chars().rev();

    let mut front_value: Option<char> = None;
    let mut back_value: Option<char> = None;

    while let (Some(front), Some(back)) = (front_iter.next(), back_iter.next()) {
        if front.is_numeric() && front_value.is_none() {
            front_value = Some(front);
        }
        if back.is_numeric() && back_value.is_none() {
            back_value = Some(back);
        }

        // Break early if both values are found.
        if front_value.is_some() && back_value.is_some() {
            break;
        }
    }

    if let (Some(front), Some(back)) = (front_value, back_value) {
        let combined = format!("{}{}", front, back);
        combined
            .parse::<i32>()
            .map_err(|_| "Failed to parse combined string as i32")
    } else {
        Err("Did not find numeric characters at both ends")
    }
}

fn p1() {
    let mut sum = 0;

    let lines = file_reader::read_lines("src/day_1/input.txt").unwrap();
    println!("{:?}", lines);

    for line in lines {
        sum += find_numeric_chars(&line).unwrap();
    }

    println!("Part1: {}", sum);
}

#[derive(Debug)]
struct KeyValue {
    key: &'static str,
    value: &'static str,
}

fn p2() {
    let mut sum = 0;
    let lines = file_reader::read_lines("src/day_1/input.txt").unwrap();

    let string_array: [KeyValue; 9] = [
        KeyValue {
            key: "one",
            value: "o1e",
        },
        KeyValue {
            key: "two",
            value: "t2o",
        },
        KeyValue {
            key: "three",
            value: "t3e",
        },
        KeyValue {
            key: "four",
            value: "f4r",
        },
        KeyValue {
            key: "five",
            value: "f5e",
        },
        KeyValue {
            key: "six",
            value: "s6x",
        },
        KeyValue {
            key: "seven",
            value: "s7n",
        },
        KeyValue {
            key: "eight",
            value: "e8t",
        },
        KeyValue {
            key: "nine",
            value: "n9e",
        },
    ];

    for mut line in lines {
        for num_str in string_array.iter() {
            line = line.replace(num_str.key, num_str.value);
        }
        sum += find_numeric_chars(&line).unwrap();
    }

    println!("Part2: {}", sum);
}

pub struct Day1;

impl Runnable for Day1 {
    fn run(&self) -> Result<(), std::io::Error> {
        p1();
        p2();
        Ok(())
    }
}