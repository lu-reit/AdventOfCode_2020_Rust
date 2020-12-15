use std::collections::HashMap;

static TEST: [usize; 3] = [0, 3, 6];
static INPUT: [usize; 7] = [6,19,0,5,7,13,1];


fn part1(vals: &[usize]) -> usize { 
    let mut num_count: HashMap<usize, usize> = HashMap::new();
    for (i, val) in vals.iter().enumerate() {
        num_count.insert(*val, i + 1);
    }

    let mut last = *vals.iter().last().unwrap();
    let mut is_first = true;
    for i in (vals.len() + 1)..=10 {
        let current = if is_first { 0 } else { i - num_count[&last] };
        println!("{}, {}", i, current);
        if !num_count.contains_key(&current) {
            is_first = true;
            num_count.insert(current, i);
        } else {
            is_first = false;
            *num_count.get_mut(&current).unwrap() = i;
        }
        last = current;
    }
    last
}

fn main() {
    let p1_result = part1(&TEST);
}
