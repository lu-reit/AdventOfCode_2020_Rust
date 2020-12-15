use std::collections::HashMap;
use std::time::Instant;
use std::mem::size_of_val;
use std::mem::size_of;


static TEST: [usize; 3] = [0, 3, 6];
static INPUT: [usize; 7] = [6,19,0,5,7,13,1];


fn find_nth(nth: usize, vals: &[usize]) -> usize { 
    // Populate a HashMap with the initial values
    let mut num_count: HashMap<usize, usize> = HashMap::new();
    for (i, val) in vals.iter().enumerate() {
        num_count.insert(*val, i + 1);
    }

    // Initialize last and is_first (keeps track 
    // of whether we have already that number)
    let mut last = *vals.iter().last().unwrap();
    let mut is_first = true;
    for i in (vals.len() + 1)..=nth {
        let current = if is_first { 0 } else { i - 1 - num_count[&last] };

        // Keep track of whether we have already seen that number
        if !num_count.contains_key(&current) {
            is_first = true;
        } else {
            is_first = false;
        }
        // Update the entry for the previous number 
        *num_count.entry(last).or_insert(i - 1) = i - 1;
        last = current;
    }
    println!("Size of HashMap: {}", num_count.len() * 2 * size_of::<usize>());
    last
}

// Same as above, just using a vector instead of a hashmap
fn find_nth_vec(nth: usize, vals: &[usize]) -> usize { 
    let mut num_count: Vec<usize> = vec![0; nth];
    for (i, val) in vals.iter().enumerate() {
        num_count[*val] = i + 1;
    }

    let mut last = *vals.iter().last().unwrap();
    let mut is_first = true;

    for i in (vals.len() + 1)..=nth {
        let current = if is_first { 0 } else { i - 1 - num_count[last] };

        // Keep track of whether we have already seen that number
        if num_count[current] == 0 {
            is_first = true;
        } else {
            is_first = false;
        }
        // Update the entry for the previous number 
        num_count[last] = i - 1;
        last = current;
    }
    println!("Size of Vector: {}", size_of_val(&*num_count));
    last
}

fn main() {
    let p1_result = find_nth_vec(2020, &INPUT);
    println!("Part1 result: {}", p1_result);
    println!();

    find_nth(30000000, &INPUT);
    let p2_timer = Instant::now();
    let p2_result = find_nth_vec(30000000, &INPUT);
    let p2_time = p2_timer.elapsed();
    println!("Part2 result: {}", p2_result);
    println!("Part2 time: {:?}", p2_time);
}
