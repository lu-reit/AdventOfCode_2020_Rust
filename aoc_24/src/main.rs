use std::fs;
use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let p1_timer = Instant::now();
    let cell_set = parse_directions("input");
    let p1_result = cell_set.len();
    let p1_time = p1_timer.elapsed();
    println!("Result part1: {}", p1_result);
    println!("Time part1: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = run_n_times(100, cell_set);
    let p2_time = p2_timer.elapsed();
    println!("Result part2: {}", p2_result);
    println!("Time part2: {:?}", p2_time);
    
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cell {
    x: i32,
    y: i32
}

impl Cell {
    fn new(x: i32, y: i32) -> Cell {
        Cell { x, y }
    }

    // Get a list of adjacent cells. Uses axial
    // coordinate system.
    fn get_adjacent(&self) -> Vec<Cell> {
        let mut adjacent: Vec<Cell> = Vec::new();
        
        // East
        adjacent.push(Cell::new(self.x + 1, self.y));
        // West 
        adjacent.push(Cell::new(self.x - 1, self.y));
        // North-west 
        adjacent.push(Cell::new(self.x, self.y - 1));
        // North-east
        adjacent.push(Cell::new(self.x + 1, self.y - 1));
        // South-west
        adjacent.push(Cell::new(self.x - 1, self.y + 1));
        // South-east 
        adjacent.push(Cell::new(self.x, self.y + 1));

        adjacent
    }
}

fn run_n_times(times: usize, mut cell_set: HashSet<Cell>) -> usize { 
    for i in 0..times {
        cell_set = run_once(&cell_set);
    }
    cell_set.len()
}


fn run_once(cell_set: &HashSet<Cell>) -> HashSet<Cell> {
    let all_adjacent = get_all_adjacent(&cell_set);
    let flipped_blacks = handle_blacks(&all_adjacent, &cell_set);
    let flipped_whites = handle_whites(&all_adjacent, &cell_set);
    flipped_blacks.union(&flipped_whites).copied().collect()
}

fn get_all_adjacent(cell_set: &HashSet<Cell>) -> Vec<(Cell, Vec<Cell>)> {
    let mut adjacent: Vec<(Cell, Vec<Cell>)> = Vec::new();

    for cell in cell_set.iter() {
        adjacent.push((*cell, cell.get_adjacent()));
    }
    adjacent
}

fn handle_blacks(adjacents: &Vec<(Cell, Vec<Cell>)>, cell_set: &HashSet<Cell>)
    -> HashSet<Cell> {
    let mut new_blacks: HashSet<Cell> = HashSet::new();

    for (cell, adjacent_cells) in adjacents {
        let mut adjacent_count = 0;

        for adjacent_cell in adjacent_cells {
            if cell_set.contains(adjacent_cell) {
                adjacent_count += 1;
            }
        }
        if adjacent_count == 1 || adjacent_count == 2 {
            new_blacks.insert(*cell);
        }
    }
    new_blacks
}

fn handle_whites(adjacents: &Vec<(Cell, Vec<Cell>)>, cell_set: &HashSet<Cell>)
    -> HashSet<Cell> {
    let mut seen: HashSet<Cell> = HashSet::new();
    let mut new_blacks: HashSet<Cell> = HashSet::new();

    for (_, adjacent_cells) in adjacents {
        for adjacent_cell in adjacent_cells {
            if seen.contains(adjacent_cell) {
                continue
            } else {
                seen.insert(*adjacent_cell);
            }
            if cell_set.contains(adjacent_cell) {
                continue
            }

            let mut adjacent_count = 0;
            let white_adjacents = adjacent_cell.get_adjacent(); 
            for white_adjacent in white_adjacents.iter() {
                if cell_set.contains(white_adjacent) {
                    adjacent_count += 1
                }
            }
            
            if adjacent_count == 2 {
                new_blacks.insert(*adjacent_cell);
            }
        }
    }
    new_blacks
}

fn parse_directions(filename: &str) -> HashSet<Cell> {
    let mut buffer = fs::read_to_string(filename).unwrap();
    let mut cell_set = HashSet::new();
    
    for dir_list in buffer.trim().split('\n') {
        let mut cell = Cell::new(0, 0);
        let mut chars_iter = dir_list.chars();
        
        loop { 
            match chars_iter.next() {
                Some(chr) if chr == 'e' =>  { cell.x += 1; }
                Some(chr) if chr == 'w' => { cell.x -= 1; }
                Some(chr) if chr == 'n' => {
                    cell.y -= 1;
                    let next = chars_iter.next().unwrap();
                    if next == 'e' { cell.x += 1; } 
                }
                Some(chr) if chr == 's' => {
                    cell.y += 1;
                    let next = chars_iter.next().unwrap();
                    if next == 'w' { cell.x -= 1; } 
                }
                Some(_) =>  { panic!("Unknown character encountered"); }
                None => { break }
            }
        }
        if cell_set.contains(&cell) {
            cell_set.remove(&cell); 
        } else {
            cell_set.insert(cell);
        }
    }
    cell_set
}
