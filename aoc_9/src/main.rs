use std::fs;
use std::time::Instant;

fn main() {
    let (file, pre_size) = ("input", 25);
    let numbers = read_numbers(file);


    let p1_timer = Instant::now();
    let p1_result = part1(pre_size, &numbers);
    let p1_time = p1_timer.elapsed();
    println!("Result part1: {:?}", p1_result);
    println!("Time part1: {:?}", p1_time);
    
}

fn part1(pre_size: usize, numbers: &[u64]) -> Option<u64> {
    'b_loop: for batch in numbers.windows(pre_size + 1).skip(1) {
        for x in batch[..pre_size].iter() {
            for y in batch[..pre_size].iter().filter(|&y| y != x) {
                if y + x == batch[pre_size] { continue 'b_loop; } 
            }
        }
        return Some(batch[pre_size]);
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
 

