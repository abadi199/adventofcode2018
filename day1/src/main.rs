use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let frequency = BufReader::new(File::open("input.txt").expect("Unable to open input.txt"))
        .lines()
        .map(|result| result.unwrap_or("".to_string()).parse::<i32>().unwrap_or(0))
        .fold(0, |frequency_change, current_frequency| {
            current_frequency + frequency_change
        });

    println!("Frequency: {}", frequency);
}
