use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() -> io::Result<()> {
    println!("Day 01 part 2 code is running!");
    let path = "src/day01/input.txt"; // Replace with the path to your input file
    let sum = sum_calibration_values(path)?;
    println!("The sum of the calibration values is: {}", sum);
    Ok(())
}

fn sum_calibration_values<P: AsRef<Path>>(path: P) -> io::Result<i32> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        let line = line?;
        if let Some(value) = extract_calibration_value(&line) {
            sum += value;
        }
    }
    Ok(sum)
}

fn extract_calibration_value(line: &str) -> Option<i32> {
    let spelled_digits = [
        ("zero", '0'), ("one", '1'), ("two", '2'), ("three", '3'), 
        ("four", '4'), ("five", '5'), ("six", '6'), ("seven", '7'), 
        ("eight", '8'), ("nine", '9')
    ];
    let mut current_string = String::new();
    let mut first_digit = None;
    let mut last_digit = None;

    for ch in line.chars() {
        if ch.is_digit(10) {
            if first_digit.is_none() {
                first_digit = Some(ch);
            }
            last_digit = Some(ch);
            current_string.clear();
        } else {
            current_string.push(ch);
            for &(word, digit) in &spelled_digits {
                if current_string.ends_with(word) {
                    if first_digit.is_none() {
                        first_digit = Some(digit);
                    }
                    last_digit = Some(digit);
                    break;
                }
            }
        }
    }

    match (first_digit, last_digit) {
        (Some(f), Some(l)) => format!("{}{}", f, l).parse::<i32>().ok(),
        _ => None,
    }
}

