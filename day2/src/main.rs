use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let product_ids = read_product_ids();
    let twos = product_ids
        .iter()
        .filter(|id| has_exactly_n_letters(id, 2))
        .count();
    let threes = product_ids
        .iter()
        .filter(|id| has_exactly_n_letters(id, 3))
        .count();
    let checksum = twos * threes;

    println!("twos: {}", twos);
    println!("threes: {}", threes);
    println!("checksum: {}", checksum);
}

fn read_product_ids() -> Vec<String> {
    BufReader::new(File::open("input.txt").expect("Unable to open input.txt"))
        .lines()
        .filter_map(|result| result.ok())
        .collect()
}

fn has_exactly_n_letters(id: &str, n: u32) -> bool {
    let mut counter: HashMap<char, u32> = HashMap::new();
    for letter in id.chars() {
        let count: u32 = counter.get(&letter.clone()).map(|c| c + 1).unwrap_or(1);
        counter.insert(letter.clone(), count);
    }

    for (_, count) in counter {
        if count == n {
            return true;
        }
    }

    false
}
