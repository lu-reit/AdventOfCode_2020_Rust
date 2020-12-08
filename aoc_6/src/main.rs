use std::fs;
use std::time::Instant;
extern crate fxhash;

fn main() {
    let answers = read_file("input");
    let timer = Instant::now();
    let result = both_parts(&answers);
    let elapsed = timer.elapsed();
    println!("Both parts: {:?}", result);
    println!("Time needed: {:?}", elapsed);
}

fn both_parts(answers: &Vec<String>) -> (usize, usize) {
    let mut uni_person: usize = 0;
    let mut uni_group: usize = 0;


    let mut seen = fxhash::FxHashMap::default();
    for answer in answers {
        let mut n_person: usize = 1;

        for chr in answer.chars() {
            if chr.is_alphabetic() { *seen.entry(chr).or_insert(0) += 1; }
            else if chr == '\n' { n_person += 1; }
        }
        uni_person += seen.len();
        uni_group += seen.iter().filter(|(_, x)| **x == n_person).count();
        seen.clear()
    }
    (uni_person, uni_group)
}

fn read_file(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .trim()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect()
}
