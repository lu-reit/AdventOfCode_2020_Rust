use std::fs;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct ChairMap {
    width: usize,
    height: usize,
    // None -> no seat; Some(false) -> empty seat, Some(true) -> occupied seat 
    map: Vec<Option<bool>>, 
    adjacancy: Vec<Vec<usize>>, // List of all adjacent seats 
    variants: Vec<usize> // List of seats that can still change
}

impl ChairMap {
    fn apply_rules(&self, cord: usize) -> Option<bool> {
        match self.map[cord] {
            Some(occupied) if occupied => { 
                let n_seated = self.adjacancy[cord].iter()
                    .filter(|&&adj_cord| self.map[adj_cord] == Some(true))
                    .count();
                if n_seated > 3 { Some(false) } else { Some(true) }
            }
            Some(empty) => { Some(empty) }
            None => { None }
        }
    }

    fn run_once(&mut self) {
        
        self.variants.iter()
            map(|
            


        }
    }

    fn print_map(&self) {
        for i in 0..self.height {
            let row = i * self.width;
            let to_print: String = self.map[row..(row + self.width)].iter()
                .map(|x| { 
                    match x {
                        None => '.',
                        Some(seat) if *seat => '#',
                        _ => 'L'
                    }
                })
                .collect();
            println!("{}", to_print);
        }
        println!("");
    }
}

        



fn main() {
    let chairmap = build_map(&read_grid("test"));
    chairmap.print_map()
}

fn build_map(chr_grid: &Vec<Vec<u8>>) -> ChairMap {
    let width = chr_grid[0].len();
    let height = chr_grid.len();
    let n_elem = width * height;
    let mut map: Vec<Option<bool>> = Vec::with_capacity(n_elem);
    let mut variants: Vec<usize> = Vec::with_capacity(n_elem);
    let mut adjacancy: Vec<Vec<usize>> = Vec::with_capacity(n_elem);

    for (y, chr_row) in chr_grid.iter().enumerate() {
        for (x, chr) in chr_row.iter().enumerate() {
            let adjacent = get_adjacancy(x, y, width, height, &chr_grid);

            match chr { 
                b'.' => { map.push(None); }
                b'L' => { 
                    // Start with everyone seated
                    map.push(Some(true)); 
                    // Anyone seated with less than 4 neighbors will stay seated
                    if adjacent.len() > 3 { variants.push(y * width + x); }
                }
                _ => { panic!("Failed building map"); 
                }
            }
            adjacancy.push(adjacent);
        }
    }
    ChairMap { width, height, map, variants, adjacancy }
}

// Check whether a position is still inside the grid
fn in_bounds(x: i32, y: i32, width: usize, height: usize) -> bool {
    x >= 0 && x < (width as i32) && y >= 0 && y < (height as i32)
}

// Get a list of all adjacent seats (not empty spots)
fn get_adjacancy(x: usize, y: usize, width: usize, height: usize,
    map: &Vec<Vec<u8>>) -> Vec<usize> {
    let mut adjacancy: Vec<usize> = Vec::new();

    for dy in -1..=1 {
        for dx in -1..=1 {
            // Need to use signed integers here otherwise this will overflow
            let (x_n, y_n) = ((x as i32) + dx, (y as i32) + dy);

            // Cant be itself/out_of_bounds and must be a seat
            if !(dx == 0 && dy == 0) && in_bounds(x_n, y_n, width, height)  
                && map[x_n as usize][y_n as usize] != b'.' {
                adjacancy.push((x_n as usize) + (y_n as usize) * width);
            }
        }
    }
    adjacancy
}

fn read_grid(filename: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(filename).unwrap()
        .trim()
        .split('\n')
        .map(|s| s.as_bytes().to_vec())
        .collect()
}
