use std::fs;
use std::time::Instant;

mod vector;
use vector::Vector;
use vector::Scalar;

type Action = (char, Scalar);

struct Vessel {
    pos: Vector,
    head: usize
}

// Principal directions
// Cant use Vector::new() here...
static HEADS: [Vector; 4] = [Vector { x: 1, y: 0 }, Vector { x: 0, y: 1 },
                            Vector { x: -1, y: 0 }, Vector { x: 0, y: -1 }];

fn main() {
    let actions = read_actions("input");
    
    let p1_timer = Instant::now();
    let p1_result = part1(&actions);
    let p1_time = p1_timer.elapsed();
    println!("Part1 result: {}", p1_result);
    println!("Part1 time: {:?}", p1_time);
}

fn part1(actions: &[Action]) -> Scalar {
    let mut boat = Vessel { pos: Vector::new(0, 0), head: 0 };
    actions.iter().for_each(|action| act(action, &mut boat));
    boat.pos.dist()
}

fn part2(actions: &[Action]) -> Scalar {
    let mut boat = Vessel { pos: Vector::new(0,0), head, 0 }

}

fn act_p1(action: &Action, vessel: &mut Vessel)  {
    match action.0 {
        'E' => { vessel.pos = vessel.pos + Vector::new(action.1, 0) }
        'W' => { vessel.pos = vessel.pos + Vector::new(-action.1, 0) }
        'N' => { vessel.pos = vessel.pos + Vector::new(0, action.1) }
        'S' => { vessel.pos = vessel.pos + Vector::new(0, -action.1) }
        'L' | 'R' => { change_heading(action, vessel) }
        'F' => { vessel.pos = vessel.pos + action.1 * HEADS[vessel.head] }
        _ => panic!("Illegal Action encountered")
    };
}

fn change_heading(action: &Action, vessel: &mut Vessel) {
    let turns = match action.1 {
        90 => 1,
        180 => 2,
        270 => 3,
        _ => panic!("Illegal turn encountered")
    };
    match action.0 {
        'L' => vessel.head = (vessel.head + offset) % 4,
        'R' => vessel.head = (4 - offset + vessel.head) % 4,
        _ => unreachable!()
    };
}

fn read_actions(filename: &str) -> Vec<Action> {
    fs::read_to_string(filename).unwrap()
        .trim()
        .split('\n')
        .map(|s| 
            (s.chars().next().unwrap(),
            s[1..].parse::<Scalar>().unwrap())
        )
        .collect()
}
