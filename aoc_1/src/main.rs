use std::fs::File;
use std::io::Read;


fn main() {
    let mut nums: Vec<i64> = read_file("input");
    
    // Sorting the numbers allows some optimizations
    nums.sort();

    println!("Result part1: {}", part1(&nums));
    println!("Result part2: {}", part2(&nums));
}
 
fn part1(nums: &Vec<i64>) -> i64   {
    for x in nums.iter() {
        let diffx = 2020 - x; 
        
        for y in nums.iter() {

            // Break early to avoid unneccesary iterations
            if *y > diffx { break }
            if x + y == 2020 { return x * y }
        }
    }
    0 // Shut up rust
}


fn part2(nums: &Vec<i64>) -> i64 {
    for x in nums.iter() {
        let diffx = 2020 - x; 

        for y in nums.iter() {
            if *y > diffx { break }
            let diffxy = diffx - y;

            for z in nums.iter() {
                if *z > diffxy { break } 
                if x + y + z == 2020 { return x * y * z }
            }
        }
    }
    0
}

fn read_file(filename: &str) -> Vec<i64> {
    let mut buffer: String = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut buffer).unwrap();

    let nums: Vec<i64> = buffer.trim()
        .split("\n")
        .map(|s| s.parse::<i64>().expect("Couldnt parse the input"))
        .collect();

    nums
}


