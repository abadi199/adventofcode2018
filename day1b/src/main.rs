use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let frequency_changes = read_file();
    let twice = twice(&frequency_changes);

    println!("Twice: {:?}", twice);
}

fn read_file() -> Vec<i32> {
    BufReader::new(File::open("input.txt").expect("Unable to open input.txt"))
        .lines()
        .map(|result| {
            result
                .ok()
                .and_then(|line| line.parse::<i32>().ok())
                .unwrap_or(0)
        }).collect()
}

fn twice(frequency_changes: &[i32]) -> i32 {
    let mut frequency = 0;
    let mut frequency_set: HashSet<i32> = HashSet::new();
    loop {
        for frequency_change in frequency_changes.iter() {
            frequency += frequency_change;
            if !frequency_set.insert(frequency) {
                return frequency;
            }
        }
    }
}
