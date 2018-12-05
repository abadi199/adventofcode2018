use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let product_ids = read_product_ids();
    let (twos, threes) = product_ids
        .iter()
        .map(|id| count_letters(id))
        .fold((0, 0), calculate_two_and_three);
    let checksum = twos * threes;
    println!("checksum: {}", checksum);
}

fn read_product_ids() -> Vec<String> {
    BufReader::new(File::open("input.txt").expect("Unable to open input.txt"))
        .lines()
        .filter_map(|result| result.ok())
        .collect()
}

fn count_letters(id: &str) -> HashMap<char, u32> {
    let mut counter: HashMap<char, u32> = HashMap::new();
    for letter in id.chars() {
        let count: u32 = counter.get(&letter.clone()).map(|c| c + 1).unwrap_or(1);
        counter.insert(letter.clone(), count);
    }

    counter
}

fn calculate_two_and_three((two, three): (u32, u32), counter: HashMap<char, u32>) -> (u32, u32) {
    let two = match has_exactly_n_letter(&counter, 2) {
        true => two + 1,
        false => two,
    };

    let three = match has_exactly_n_letter(&counter, 3) {
        true => three + 1,
        false => three,
    };

    (two, three)
}

fn has_exactly_n_letter(counter: &HashMap<char, u32>, n: u32) -> bool {
    for (_, count) in counter {
        if count == &n {
            return true;
        }
    }

    false
}
