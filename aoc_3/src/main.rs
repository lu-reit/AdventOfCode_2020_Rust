use std::fs;
use std::time::Instant;

fn main() {
    let treemap = read_file("input");

    let p1_timer = Instant::now();
    let p1_result = trees_encountered((3, 1), &treemap);
    let p1_time = p1_timer.elapsed();
    let p2_timer = Instant::now();
    let p2_result = part2(&treemap);
    let p2_time = p2_timer.elapsed();

    println!("Result part1: {}", p1_result);
    println!("Part1 time: {:?}", p1_time);
    println!("Result part2: {}", p2_result);
    println!("Part2 time: {:?}", p2_time);
}

#[derive(Debug)]
struct TreeMap {
    height: usize,
    width: usize,
    trees: Vec<usize>
}

fn part2(treemap: &TreeMap) -> usize {
    let slopes: Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes.iter().map(|slope| trees_encountered(*slope, &treemap)).product()
}

fn trees_encountered(slope: (usize, usize), treemap: &TreeMap) -> usize {
    let (mut posx, mut posy) = (0, 0);
    let mut n_trees = 0;
    while posy < treemap.height {
        n_trees += treemap.trees[posx + posy * treemap.width];
        posx = (posx + slope.0) % treemap.width;
        posy += slope.1;
    }
    n_trees
}

fn read_file(filename: &str) -> TreeMap {
    let buffer = fs::read_to_string(filename).unwrap();
    let width = buffer.lines().next().unwrap().len();

    let mut height = 0;
    let mut trees: Vec<usize> = Vec::with_capacity(buffer.len());
    for line in buffer.lines() {
        for chr in line.chars() {
            if chr == '#' { trees.push(1) }
            else { trees.push(0) }
        }
        height += 1;
    }
    TreeMap { height, width, trees }
}
