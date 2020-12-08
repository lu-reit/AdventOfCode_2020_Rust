use std::fs;

fn main() {
    let ids = decode_file("input");
    println!("Result of part1: {}", ids.last().unwrap());
    println!("Result of part2: {}", part2(&ids));
}
 
fn part2(ids: &Vec<u16>) -> u16 {
    for (i, id) in ids.iter().enumerate().skip(1) {
        if (id - ids[i - 1]) > 1 { return id - 1; }
    }
    panic!("Id not found");
}

fn get_id(pattern: &str) -> u16 {
    // Build a int by shiftings the bits in place
    pattern.chars().rev().enumerate()
        .fold(0, |int, (i, chr)| {
            if chr == 'B' || chr == 'R' { int | 1 << i } else { int }
        })
}

fn decode_file(filename: &str) -> Vec<u16> {
    let mut ids: Vec<u16> = fs::read_to_string(filename).unwrap()
        .trim()
        .split("\n")
        .map(get_id)
        .collect();
    ids.sort(); // Sorting makes everything easier
    ids
}
