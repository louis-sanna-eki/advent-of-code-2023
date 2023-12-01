use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run() -> io::Result<()> {
    println!("Day 01 code is running!");
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
    let first_digit = line.chars().find(|c| c.is_digit(10));
    let last_digit = line.chars().rev().find(|c| c.is_digit(10));

    match (first_digit, last_digit) {
        (Some(f), Some(l)) => {
            let value = format!("{}{}", f, l).parse::<i32>().ok()?;
            Some(value)
        }
        _ => None,
    }
}
