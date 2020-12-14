use std::fs;
use std::u64;
use regex::Regex;
use std::time::Instant;
use std::collections::HashSet;

fn main() {
    let memrules = read_file("input");

    // Initialize the memory
    // Memory addresses in the input are at most 5 digits long
    let mut memory: Vec<u64> = vec![0; 100000];

    let p1_result = part1(&memrules, &mut memory);
    println!("Part1 result: {}", p1_result);

}

type Unit = u64;
type MemVal = (usize, Unit);

#[derive(Debug, Clone)]
struct BitRule {
    k_mask: Unit,
    s_mask: Unit,
    values: Vec<MemVal>
}
impl BitRule {
    fn new() -> BitRule {
        BitRule {
            k_mask: 0,
            s_mask: 0,
            values: Vec::new()
        }
    }
}

fn part1(rules: &[BitRule], mem: &mut Vec<Unit>) -> u64 {
    // Set to keep track of all addresses that changed
    let mut addr_set: HashSet<usize> = HashSet::with_capacity(rules.len() * 5);

    // Apply the Rules to memory
    for rule in rules {
        for val in &rule.values {
            // Extract the bits to keep (AND) and set the rest of
            // the bits to the swap_masks value
            mem[val.0] = (val.1 & rule.k_mask) | rule.s_mask; 
            addr_set.insert(val.0);
        }
    }

    // Iterate over the indices again and sum up the values 
    addr_set.iter().fold(0, |acc, &i| acc + mem[i])
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




