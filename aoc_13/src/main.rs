use std::fs;
use std::time::Instant;

type ID = u64;
type Delta = u64;

fn main() {
    let (target, mut ids) = read_file("input");

    let p1_timer = Instant::now();
    let p1_result = part1(target, &ids);
    let p1_time = p1_timer.elapsed();
    println!("Part1 result: {}", p1_result);
    println!("Part1 time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&mut ids);
    let p2_time = p2_timer.elapsed();
    println!("Part2 result: {}", p2_result);
    println!("Part2 time: {:?}", p2_time);
}

fn part1(target: u64, ids: &[(Delta, ID)]) -> u64 {
    let (ind, time) = ids.iter().enumerate()
        .map(|(i, id)| (i, target - (target % id.1) + id.1))
        .min_by(|(_, t1), (_, t2)| t1.cmp(t2))
        .unwrap();

    (time - target) * ids[ind].1
}
 
fn part2(ids: &mut [(Delta, ID)]) -> u64 {
    // Sort and start with largest factors first
    ids.sort_by(|(_, x), (_, y)| y.cmp(x));
    let offset = ids[0].0;

    // Note that all the ids in the list are prime
    let mut base = ids[0].1;
    let mut time = base;
    for (d, id) in ids.iter().skip(1) {
        while (time + d - offset) % id != 0 {
            time += base;
        }
        // Skip ahead by the next factor
        base *= id;
    }
    return time - offset;
}

fn read_file(filename: &str) -> (u64, Vec<(Delta, ID)>) {
    let contents = fs::read_to_string(filename).unwrap();
    let mut parts = contents.split('\n');
    let time = parts.next().unwrap().parse::<u64>().unwrap();
    let ids: Vec<(Delta, ID)> = parts.next().unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .map(|(i, s)| (i as Delta, s.parse::<ID>().unwrap()))
        .collect();

    (time, ids)
}
