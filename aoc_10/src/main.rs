use std::fs;

fn main() {
    let mut numbers = read_numbers("input");
    let p1_result = part1(&mut numbers);
    println!("Part1 result: {}", p1_result);
}

fn part1(numbers: &mut Vec<u32>) -> u32 {
    numbers.push(0);
    numbers.sort();
    println!("{:?}", numbers);
    let (mut ones, mut threes) = (0, 1);
    for window in numbers.windows(2) {
        match window[1] - window[0] {
            3 => { threes += 1; }
            1 => { ones += 1; }
            _ => { panic!("Part1: Difference between adapters greater than 3"); }
        }
    }
    println!("ones: {}, threes: {}", ones, threes);
    ones * threes
}
    

fn read_numbers(filename: &str) -> Vec<u32> {
    fs::read_to_string(filename).unwrap()
        .trim()
        .split("\n")
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}
