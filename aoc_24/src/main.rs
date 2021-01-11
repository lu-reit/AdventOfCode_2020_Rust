use std::fs;
use std::collections::HashMap;

fn main() {
    let cell_map = parse_directions("input");
    let p1_result = part1(&cell_map);
    println!("Result part1: {}", p1_result);
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


fn part1(cell_map: &HashMap<Cell, bool>) -> usize {
    let mut blacks = 0;
    cell_map.iter()
        .fold(0, 
            |acc, (_, &is_black)| if is_black { acc + 1 } else { acc }
        )
}

fn parse_directions(filename: &str) -> HashMap<Cell, bool> {
    let mut buffer = fs::read_to_string(filename).unwrap();
    let mut cell_map = HashMap::new();
    
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
        println!("Cell: {:?}", cell);
        let entry = cell_map.entry(cell).or_insert(false);
        println!("Entry before: {}", entry);
        *entry = !(*entry);
        println!("Entry after: {}", entry);
         
    }
    cell_map
}
