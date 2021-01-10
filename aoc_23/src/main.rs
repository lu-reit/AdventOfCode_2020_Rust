use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    let p1_timer = Instant::now();
    let p1_start_labeling = [5, 2, 3, 7, 6, 4, 8, 1, 9];
    let p1_result = part1(p1_start_labeling);
    let p1_time = p1_timer.elapsed();
    println!("Part 1 result: {}", p1_result);
    println!("Part 1 time: {:?}", p1_time);

    let p2_start_labeling = [5, 4, 3, 2, 1];
    let p2_result = part2(10_000_000, &p2_start_labeling);


}

fn wrap(first: u32, offset: u32) -> u32 {
    if offset >= first {
        (9 + first) - offset
    } else {
        first - offset
    }
}

fn part1(start_labels: [u32; 9]) -> u32 {
    let mut cups: VecDeque<u32> = start_labels.iter().cloned().collect();
    for _ in 0..100 {
        let first = *cups.front().unwrap();
        cups.rotate_left(1); 
        let triple: Vec<u32> = cups.drain(0..3).collect();

        // Find a suitable Destination
        let mut dest = wrap(first, 1);
        while triple.contains(&dest) { dest = wrap(dest, 1); }
        
        // Find the position of the element that corresponds to dest
        // and insert the triple after it.
        let dest_i = cups.iter().position(|&x| x == dest).unwrap();
        for x in triple.iter().rev() {
            cups.insert(dest_i + 1, *x);
        }
    }
    let one_i = cups.iter().position(|&x| x == 1).unwrap();
    ((one_i + 1)..(one_i + 9))
        .map(|to_wrap| to_wrap % 9)
        .rev()
        .enumerate()
        .fold(0, |acc, (i, ring_i)| acc + cups[ring_i] * 10u32.pow(i as u32))
}


fn part2(n_moves: usize, labels: &[u32]) -> u64 {
    let mut cups: VecDeque<u32> = VecDeque::new();
    cups.extend(labels);
    cups.extend(
        ((labels.len() + 1)..=1_000_000)
        .map(|x| x as u32)
        .collect::<Vec<u32>>()
    );
    for move_i in 0..n_moves {
        println!("Move i: {}", move_i);

        let first = *cups.front().unwrap();
        cups.rotate_left(1); 
        let triple: Vec<u32> = cups.drain(0..3).collect();

        // Find a suitable Destination
        let mut dest = wrap(first, 1);
        while triple.contains(&dest) { dest = wrap(dest, 1); }
        
        // Find the position of the element that corresponds to dest
        // and insert the triple after it.
        let dest_i = cups.iter().position(|&x| x == dest).unwrap();
        for x in triple.iter().rev() {
            cups.insert(dest_i + 1, *x);
        }
    }
    0
}

