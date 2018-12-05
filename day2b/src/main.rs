use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let product_ids = read_product_ids();
    if let Some(answer) = first_match(product_ids) {
        println!("Answer: {:?}", answer);
    }
}

fn first_match(product_ids: Vec<String>) -> Option<String> {
    for i in 0..(product_ids.len() - 1) {
        for j in (i + 1)..(product_ids.len() - 1) {
            // TODO

            let answer = matches(&product_ids[i], &product_ids[j]);
            if answer.len() == (product_ids[i].len() - 1) {
                return Some(answer);
            }
        }
    }

    None
}

fn read_product_ids() -> Vec<String> {
    BufReader::new(File::open("input.txt").expect("Unable to open input.txt"))
        .lines()
        .filter_map(|result| result.ok())
        .collect()
}

fn matches(first: &str, second: &str) -> String {
    let mut matches: Vec<char> = Vec::new();
    let len = std::cmp::max(first.len(), second.len());
    let first_chars: Vec<char> = first.chars().collect();
    let second_chars: Vec<char> = second.chars().collect();

    for i in 0..len {
        if first_chars[i] == second_chars[i] {
            matches.push(first_chars[i]);
        }
    }

    matches.into_iter().collect()
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
