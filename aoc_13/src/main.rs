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
    let base = ids[0];
    ids.sort_by(|(_, x), (_, y)| y.cmp(x));
    let first = ids[0];
    let mut count: ID = 1;  

    'outer: loop { 
        let time = first.1 * count - first.0;
        println!("{}", time);

        for id in ids.iter() {
            if (time + id.0 as ID) % id.1 != 0 { 
                count += 1;
                continue 'outer
            }
        }
        return time + base.1 as u64;
    }
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
