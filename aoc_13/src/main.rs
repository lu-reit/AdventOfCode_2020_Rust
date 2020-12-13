use std::fs;

type ID = u64;
type Delta = u64;

fn main() {
    let (target, mut ids) = read_file("input");

    let p1_result = part1(target, &ids);
    println!("Part1 result: {}", p1_result);
    println!("{:?}", ids);

    let p2_result = part2(&mut ids);
    println!("Part2 result: {}", p2_result);
}

fn part1(target: u64, ids: &[(Delta, ID)]) -> u64 {
    let (ind, time) = ids.iter().enumerate()
        .map(|(i, id)| (i, target - (target % id.1) + id.1))
        .min_by(|(_, t1), (_, t2)| t1.cmp(t2))
        .unwrap();

    (time - target) * ids[ind].1
}
 
fn part2(ids: &mut [(Delta, ID)]) -> u64 {
    let mut base = ids[0].1;
    let mut time = 0;
    for (d, id) in ids.iter().skip(1) {
        while (time + d) % id != 0 {
            time += base;
        }
        base *= id;
    }
    return time;
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
