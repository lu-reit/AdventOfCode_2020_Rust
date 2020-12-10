use std::fs;
use std::time::Instant;

fn main() {
    let mut numbers = read_numbers("input");

    // Sort the Vec and add the input/output joltages.
    // This makes both parts much easier to solve.
    let sort_timer = Instant::now();
    numbers.push(0);
    numbers.sort();
    numbers.push(numbers.iter().last().unwrap() + 3);
    println!("Time for sorting: {:?}", sort_timer.elapsed());

    run_and_time(1, &numbers, &part1);
    run_and_time(2, &numbers, &part2);
}

// Just count the differences in the sorted list
fn part1(numbers: &[u32]) -> u64 {
    let (mut ones, mut threes) = (0, 0);

    for window in numbers.windows(2) {
        match window[1] - window[0] {
            3 => { threes += 1; }
            1 => { ones += 1; }
            _ => { panic!("Part1: Difference between adapters greater than 3"); }
        }
    }
    ones * threes
}

fn part2(numbers: &[u32]) -> u64 {
    let len = numbers.len();

    // Holds the number of possible paths that still can be taken
    let mut paths: Vec<u64> = vec![1; len];
    // Deal with the edgecase number.len() - 3 
    if numbers[len - 1] - numbers[len - 3] <= 3 { paths[len - 3] = 2; }

    // Iterate through numbers/paths in reverse and sum over the paths
    // for the previous adapters which are in range
    for i in (0..len - 3).rev() {
        let mut path_sum = paths[i + 1];
        if numbers[i + 2] - numbers[i] <= 3 { path_sum += paths[i + 2] }
        if numbers[i + 3] - numbers[i] <= 3 { path_sum += paths[i + 3] }
        paths[i] = path_sum;
    } 
    paths[0]
} 

// Playing around with functions as arguments
fn run_and_time(part: u8, numbers: &[u32], f: &dyn Fn(&[u32]) -> u64) {
    let timer = Instant::now();
    let result = f(numbers);
    let time = timer.elapsed();
    println!("Result part{}: {}", part, result);
    // Printing time as debug automatically converts time into 
    // the appropiate unit
    println!("Time part{}: {:?}\n", part, time);
}


fn read_numbers(filename: &str) -> Vec<u32> {
    fs::read_to_string(filename).unwrap()
        .trim()
        .split('\n')
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}
