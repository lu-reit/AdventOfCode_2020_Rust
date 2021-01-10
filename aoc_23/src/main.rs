use std::collections::VecDeque;

fn main() {
    let start_labeling: [u32; 9] = [5, 2, 3, 7, 6, 4, 8, 1, 9];
    part1(start_labeling);
}

fn part1(start_labels: [u32; 9]) {
    let mut cups: VecDeque<u32> = start_labels.iter().cloned().collect();
    for _ in 0..100 {
        let first = *cups.front().unwrap();
        cups.rotate_left(1); 
        let triple: Vec<u32> = cups.drain(0..3).collect();

        // Find a suitable Destination
        let mut dest = wrap(first, 1);
        while triple.contains(&dest) { dest = wrap(dest, 1); }
        
        // Find the element that corresponds to dest and 
        // insert the triple after it
        for i in 0..6 {
            if cups[i] == dest {
                // Iterating in reverse so that we do not have to
                // change the index
                for x in triple.iter().rev() {
                    cups.insert(i + 1, *x);
                }
                break;
            }
        }
    }
}

fn wrap(first: u32, offset: u32) -> u32 {
    if offset >= first {
        (9 + first) - offset
    } else {
        first - offset
    }
}


