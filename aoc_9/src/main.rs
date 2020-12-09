use std::fs;
use std::time::Instant;

// Use an optimized HashSet
extern crate fxhash;
use fxhash::FxHashSet;

fn main() {
    let (file, pre_size) = ("input", 25);
    let numbers = read_numbers(file);

    let p1_timer = Instant::now();
    let p1_result_opt = part1(pre_size, &numbers);
    let p1_time = p1_timer.elapsed();
    let p1_result = p1_result_opt.expect("Part1 failed. No point to continue");
    println!("Result part1: {:?}", p1_result);
    println!("Time part1: {:?}", p1_time);

    let p1_alt_timer = Instant::now();
    let p1_alt_result_opt = part1_alt(pre_size, &numbers);
    let p1_alt_time = p1_alt_timer.elapsed();
    let p1_alt_result = p1_alt_result_opt.expect("Part1 failed. No point to continue");
    println!("Result part1_alt: {:?}", p1_alt_result);
    println!("Time part1_alt: {:?}", p1_alt_time);

    let p2_timer = Instant::now();
    let p2_result_opt = part2(p1_result, &numbers);
    let p2_time = p2_timer.elapsed();
    let p2_result = p2_result_opt.expect("Part1 failed. No point to continue");
    println!("Result part2: {:?}", p2_result);
    println!("Time part2: {:?}", p2_time);
}

fn part1(pre_size: usize, numbers: &[u64]) -> Option<u64> {
    'b_loop: for batch in numbers.windows(pre_size + 1).skip(1) {
        for x in batch[..pre_size].iter() {
            for y in batch[..pre_size].iter().filter(|&y| y != x) {
                // If a sum has been found, we can ignore the entire batch
                if y + x == batch[pre_size] { continue 'b_loop; } 
            }
        }
        // We can only arrive here if the inner loops finished running
        return Some(batch[pre_size]);
    }
    None        
}

// Alternative Solution, using a HashSet, with a complexity of O(n).
// However, for the given input this ends being slower than the O(n^2) solution 
// above (Much slower if you use the HashSet from the standart library)
fn part1_alt(pre_size: usize, numbers: &[u64]) -> Option<u64> {
    let mut batch_set: FxHashSet<u64> = FxHashSet::default();

    'b_loop: for batch in numbers.windows(pre_size + 1).skip(1) {
        let target = &batch[pre_size];
        batch_set.extend(batch);

        for x in batch[..pre_size].iter() {
            if x > target { continue }
            if batch_set.contains(&(target - x)) { 
                batch_set.clear();
                continue 'b_loop }
        }
        return Some(batch[pre_size]);
    }
    None        
}
 

fn part2(target: u64, numbers: &[u64]) -> Option<u64> {
    // Do not need to consider any values which have indices greater than target
    let high: usize = numbers.iter().position(|&x| x == target).unwrap();

    // Calculate all cumulative Sums
    let mut cumm_sum: u64 = 0;
    let mut cumms: Vec<u64> = Vec::with_capacity(high);
    for x in &numbers[..high] { 
        cumm_sum += x;
        cumms.push(cumm_sum);
    }

    // Loop over all possible window sizes
    for window in 1..high {
        for i in window..high {
            // Get the sum for current window size and position
            let sum = cumms[i] - cumms[i - window];

            if target == sum { 
                let batch = &numbers[(i - window)..=i];
                return Some(batch.iter().max().unwrap() 
                    + batch.iter().min().unwrap());
            }
            // 
            if sum > target { break }
        }
    }
    None
}

fn read_numbers(filename: &str) -> Vec<u64> {
    fs::read_to_string(filename).unwrap()
        .trim()
        .split("\n")
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}
