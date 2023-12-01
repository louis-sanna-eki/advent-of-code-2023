mod day01;

fn main() {
    match day01::solve::run() {
        Ok(_) => println!("Day 01 completed successfully"),
        Err(e) => println!("An error occurred: {}", e),
    }
}
