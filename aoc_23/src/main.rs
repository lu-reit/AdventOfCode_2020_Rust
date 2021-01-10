use std::collections::VecDeque;
use std::time::Instant;

fn main() {
    let input_labeling = [5, 2, 3, 7, 6, 4, 8, 1, 9];
    let test_labeling = [3, 8, 9, 1, 2, 5, 4, 6, 7];

    let p1_timer = Instant::now();
    let p1_result = part1(&input_labeling);
    let p1_time = p1_timer.elapsed();
    println!("Part 1 result: {}", p1_result);
    println!("Part 1 time: {:?}", p1_time);

    let p2_result = part2(10_000_000, 1_000_000,  &input_labeling);
    println!("Part 2 result: {:?}", p2_result);


}

fn wrap(first: usize, offset: usize, max: usize) -> usize {
    if offset >= first {
        (max + first) - offset
    } else {
        first - offset
    }
}

fn part1(start_labels: &[usize; 9]) -> usize {
    let mut cups: VecDeque<usize> = start_labels.iter().cloned().collect();
    for _ in 0..100 {
        let first = *cups.front().unwrap();
        cups.rotate_left(1); 
        let triple: Vec<usize> = cups.drain(0..3).collect();

        // Find a suitable Destination
        let mut dest = wrap(first, 1, 9);
        while triple.contains(&dest) { dest = wrap(dest, 1, 9); }
        
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
        .fold(0, |acc, (i, ring_i)| acc + cups[ring_i] * 10usize.pow(i as u32))
}


fn part2(n_moves: usize, n_cups: usize, initial_labels: &[usize]) -> usize {
    let cups = extend_cups(n_cups, initial_labels);
    let mut int_map: Vec<usize> = vec![0; cups.len() + 1];
    for (i, &cup) in cups.iter().enumerate() {
        int_map[cup] = cups[(i + 1) % cups.len()];
    }

    let mut window = [0; 3];
    let mut first = cups[0];
    for move_i in 0..n_moves {
        window[0] = int_map[first];
        window[1] = int_map[window[0]];
        window[2] = int_map[window[1]];

        let mut dest = wrap(first, 1, n_cups);
        while window.contains(&dest) { dest = wrap(dest, 1, n_cups); }
        
        let connector = int_map[dest];

        int_map[first] = int_map[window[2]];
        int_map[dest] = window[0];
        int_map[window[2]] = connector;
        first = int_map[first];
    }
    let next_1 = int_map[1];
    let next_2 = int_map[next_1];
    next_1 * next_2
}

fn extend_cups(n_cups: usize, initial_cups: &[usize]) -> Vec<usize> {
    let mut cups: Vec<usize> = Vec::new();
    cups.extend(initial_cups);
    cups.extend(
        ((initial_cups.len() + 1)..=1_000_000)
        .collect::<Vec<usize>>()
    );
    cups
}
