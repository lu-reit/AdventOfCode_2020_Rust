use std::fs;
use std::time::Instant;

mod vector;
use vector::Vector;
use vector::Scalar;

type Action = (char, Scalar);

struct Vessel {
    pos: Vector,
    head: Vector
}



// Principal directions
static LTURNS: [fn(Vector) -> Vector; 3] = 
    [|v: Vector| Vector { x: -v.y, y: v.x },
     |v: Vector| Vector { x: -v.x, y: -v.y },
     |v: Vector| Vector { x: v.y, y: -v.x }];



fn main() {
    let actions = read_actions("input");
    
    let p1_timer = Instant::now();
    let p1_result = part1(&actions);
    let p1_time = p1_timer.elapsed();
    println!("Part1 result: {}", p1_result);
    println!("Part1 time: {:?}", p1_time);
    println!();

    let p2_timer = Instant::now();
    let p2_result = part2(&actions);
    let p2_time = p2_timer.elapsed();
    println!("Part2 result: {}", p2_result);
    println!("Part2 time: {:?}", p2_time);
}

fn part1(actions: &[Action]) -> Scalar {
    let mut boat = Vessel { pos: Vector::new(0, 0), head: Vector::new(1, 0) };
    actions.iter().for_each(|action| act_broken(action, &mut boat));
    boat.pos.dist()
}

fn part2(actions: &[Action]) -> Scalar {
    let mut boat = Vessel { pos: Vector::new(0, 0), head: Vector::new(10, 1) };
    actions.iter().for_each(|action| act(action, &mut boat));
    boat.pos.dist()
}

fn act_broken(action: &Action, vessel: &mut Vessel)  {
    match action.0 {
        'E' => { vessel.pos = vessel.pos + Vector::new(action.1, 0) }
        'W' => { vessel.pos = vessel.pos + Vector::new(-action.1, 0) }
        'N' => { vessel.pos = vessel.pos + Vector::new(0, action.1) }
        'S' => { vessel.pos = vessel.pos + Vector::new(0, -action.1) }
        'L' | 'R' => { change_heading(action, vessel) }
        'F' => { vessel.pos = vessel.pos +  action.1 * vessel.head }
        _ => panic!("Illegal Action encountered")
    };
}

fn act(action: &Action, vessel: &mut Vessel)  {
    match action.0 {
        'E' => { vessel.head = vessel.head + Vector::new(action.1, 0) }
        'W' => { vessel.head = vessel.head + Vector::new(-action.1, 0) }
        'N' => { vessel.head = vessel.head + Vector::new(0, action.1) }
        'S' => { vessel.head = vessel.head + Vector::new(0, -action.1) }
        'L' | 'R' => { change_heading(action, vessel) }
        'F' => { vessel.pos = vessel.pos +  action.1 * vessel.head }
        _ => panic!("Illegal Action encountered")
    };
}

fn change_heading(action: &Action, vessel: &mut Vessel) {
    let turns = match action.1 {
        90 => 0,
        180 => 1,
        270 => 2,
        _ => panic!("Illegal number of turns encountered")
    };
    match action.0 {
        'L' => vessel.head = LTURNS[turns](vessel.head),
        'R' => vessel.head = LTURNS[2 - turns](vessel.head),
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
