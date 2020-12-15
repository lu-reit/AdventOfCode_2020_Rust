use std::fs;
use std::u64;
use regex::Regex;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let memrules = read_file("input");

    // Initialize the memory
    // Memory addresses in the input are at most 5 digits long
    let mut memory: Vec<u64> = vec![0; 100000];

    let p1_result = part1(&memrules);
    println!("Part1 result: {}", p1_result);

    let p2_result = part2(&memrules);
    println!("Part2 result: {}", p2_result);

}

type Unit = u64;
type MemVal = (usize, Unit);

#[derive(Debug, Clone)]
struct BitRule {
    mask_str: String,
    k_mask: Unit,
    s_mask: Unit,
    values: Vec<MemVal>
}
impl BitRule {
    fn new() -> BitRule {
        BitRule {
            mask_str: String::new(),
            k_mask: 0,
            s_mask: 0,
            values: Vec::new()
        }
    }
}

fn part1(rules: &[BitRule]) -> u64 {
    // Memory map that keeps track of all addresses which have non-zero values
    let mut addr_map: HashMap<usize, Unit> = HashMap::with_capacity(rules.len() * 5);

    // Apply the Rules to memory
    for rule in rules {
        for val in &rule.values {
            // Extract the bits to keep (AND) and set the rest of
            // the bits to the swap_masks value
            let new_value = (val.1 & rule.k_mask) | rule.s_mask; 
            // Update a value if it doesnt exist
            *addr_map.entry(val.0).or_insert(new_value) = new_value;
        }
    }

    // Iterate over the indices again and sum up the values 
    addr_map.iter().fold(0, |acc, (_, v)| acc + v)
}

fn in_range(low: usize, high: usize, ranges: &mut [usize]) {
}

fn mask_bounds(mask: u64) -> Vec<(u64, u64)> {
    let mut non_x_bounds: Vec<(u64, u64)> = Vec::new();
    let mut low_bound = 0;
    let mut was_x = false;
         
    for i in 0..36 { 
        if ((1 << i) & mask) > 0 {
            if !was_x { 
                was_x = true;
                low_bound = i; 
            }
        } else {
            if was_x {
                was_x = false;
                non_x_bounds.push((low_bound, i))
            }
        }
    }
    if was_x { non_x_bounds.push((low_bound, 35)); }
    non_x_bounds
}
 
fn n_ones(low: u64, high: u64) -> u64 {
    (1 << (high - low)) - 1 | 1
}

fn in_bound(x: u64, b_min: u64, b_max: u64) -> bool {
    x >= b_min && x <= b_max 
}

fn update_range(min: u64, max: u64, val: u64, ranges: &mut VecDeque<(u64, u64, u64)>)  {
    let len = ranges.len();
    for _ in 0..len {
        let (min_x, max_x, val_x) = ranges.pop_front().unwrap();

        if min <= min_x && max >= max_x { continue }
        let min_in = in_bound(min, min_x, max_x);
        let max_in = in_bound(max, min_x, max_x);
        if min_in && max_in {
            if min != min_x { ranges.push_back((min_x, min - 1, val_x)); }
            if max != max_x { ranges.push_back((max + 1, max_x, val_x)); }
        } else if min_in {
            ranges.push_back((min_x, min - 1, val_x));
        } else if max_in { 
            ranges.push_back((max + 1, max_x, val_x));
        } else {
            ranges.push_back((min_x, max_x, val_x));
        }
    }
    ranges.push_back((min, max, val));
}



fn part2(rules: &[BitRule]) -> u64 { 
    let cutoff: u64 = (1 << 36) - 1;
    println!("{:b}" , cutoff);


    let mut ranges: VecDeque<(u64, u64, u64)> = VecDeque::new();
    for rule in rules.iter() { 
        // Negative of the keep mask
        let n_k_keep = !rule.k_mask & cutoff;
        
        let bounds = mask_bounds(rule.k_mask);
         
        for addr in rule.values.iter() {
            let result = (addr.0 as u64) & n_k_keep | rule.s_mask;
            
            let mut last_max = result;
            let mut last_xs = 0;
            let mut last_min = 0;
            for i in 0..bounds.len() {
                let (low_m, high_m) = bounds[i];
                //println!("low: {}, high: {}", low_m, high_m);
                let ones = n_ones(low_m, high_m);
                //println!("one: {:b}", ones);
                let are_x = ones << low_m;
                //println!("are_x: {}", are_x);
                let min = if low_m > 0 { result + (1 << low_m)  } else { result };
                println!("Min: {}", min);
                let max = if i != 0 { result + are_x + last_xs } else { min + are_x };
                println!("Max: {}", max);
                //println!("last_xs: {}", last_xs);
                last_xs += are_x;
                //println!("last_xs: {}", last_xs);

                update_range(min, max, addr.1, &mut ranges);
                //println!("{}", addr.1);
                //println!("{:?}", ranges);
            }
            // println!();
        }
        //println!("{:b}", rule.k_mask);
        // println!("{:?}", bounds);
    }
    ranges.iter().fold(0, |acc, (min, max, val)| acc + (max - min + 1) * val)
    
}


    
fn read_bitmask(s: &str) -> (Unit, Unit) {
    // Shave off "mask = "
    let bits = &s[7..]; 

    // Create a bit_mask wheres all X's are 1, everything else 0
    // This will be used to save the bits we do not intend to swap
    let keep_mask = u64::from_str_radix(
        &bits.replace("1", "0").replace("X", "1"), 2).unwrap();

    // Replace the X's with 0 in the swap_mask to get a valid bit-string
    let swap_mask = u64::from_str_radix(&bits.replace("X", "0"), 2).unwrap();

    (keep_mask, swap_mask)
}


fn read_file(filename: &str) -> Vec<BitRule> {
    let mut rules: Vec<BitRule> = Vec::new();
    let re = Regex::new("[0-9]+").unwrap();

    for s in fs::read_to_string(filename).unwrap().trim().split('\n') {
        if s.starts_with("mask") {
            let mut b_rule = BitRule::new();
            let (k_mask, s_mask) = read_bitmask(s);
            b_rule.k_mask = k_mask;
            b_rule.s_mask = s_mask;
            b_rule.mask_str = s.to_string();
            rules.push(b_rule);
        } else {
            let mut parts = re.find_iter(s);
            let address =  parts.next().unwrap().as_str().parse::<usize>().unwrap();
            let val = parts.next().unwrap().as_str().parse::<u64>().unwrap();
            let last  = rules.len() - 1;
            rules[last].values.push((address, val));
        }
    }
    rules
}
