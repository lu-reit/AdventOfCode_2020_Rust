use std::fs;
use std::time::Instant;
use std::collections::VecDeque;
use std::collections::HashSet;

fn main() {
    let mut statemap1 = StateMap::build_map(&read_grid("input"));
    let mut statemap2 = statemap1.clone();
    statemap2.is_part1 = false;

//    let p1_timer = Instant::now();
//    let p1_result = run_statemap(&mut statemap1);
//    let p1_time = p1_timer.elapsed();
//    println!("Result part1: {}", p1_result);
//    println!("Time part1: {:?}", p1_time);
//
//    let p2_timer = Instant::now();
//    let p2_result = run_stairmap(&mut statemap2);
//    let p2_time = p2_timer.elapsed();
//    println!("Result part2: {}", p2_result);
//    println!("Time part2: {:?}", p2_time);
}

/*
fn run_statemap(chairmap: &mut StateMap) -> usize {
    while !chairmap.variants.is_empty() {
        chairmap.run_once();
    }
    chairmap.map.iter().count()
}
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cord {
    x: i32,
    y: i32,
    z: i32,
    w: i32
}

#[derive(Debug, Clone)]
struct StateMap {
    // Only remember the active states
    map: HashSet<Cord>,
    stay_active: (usize, usize),
    activate: usize,
    is_part1: bool
}

impl StateMap {
    fn at_cord(&self, cord: &Cord) -> bool {
        self.map.contains(cord)
    }

    fn get_adjacent(&self, cord: &Cord) -> usize {
        self.get_adjacent3(cord)
    }

    fn get_adjacent3(&self, cord: &Cord) -> usize {
        let mut adjacent: usize = 0;
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                for dz in -1i32..=1 {
                    if dx == 0 && dy == 0 && dz == 0{ continue }

                    let cord = Cord { x: cord.x + dx, y: cord.y + dy, 
                                      z: cord.y + dz, w: 0 };
                    if self.at_cord(&cord) == true { adjacent += 1; }
                }
            }
        }
        adjacent 
    }

    /*
    #[inline]
    fn apply_rules(&self, cord: &Cord) -> Option<bool> {
        let adjacent = self.get_adjacent(cord);
        let cord_i = self.to_1d(cord);
        match self.map[cord_i] {
            Some(occupied) if occupied => { 
                if adjacent >= self.seat_bound { Some(false) } else { Some(true) }
            }
            Some(_) => {
                if adjacent == 0 { Some(true) } else { Some(false) }
            }
            None => { None }
        }
    }
    */

    /*
    #[inline]
    fn run_once(&mut self) {
        let var_len = self.variants.len();

        for cord in self.variants.iter() {
            let cord_i = self.to_1d(&cord);
            self.map[cord_i] = match self.map[cord_i] {
                Some(occupied) => Some(!occupied),
                _ => panic!("Encountered an empty seat in main loop")
            };
        }
    }
    */

    fn build_map(chr_grid: &[Vec<u8>]) -> StateMap {
        let mut map: HashSet<Cord> = HashSet::new();

        for (y, chr_row) in chr_grid.iter().enumerate() {
            for (x, chr) in chr_row.iter().enumerate() {
                match chr { 
                    b'.' => { }
                    b'#' => { 
                        map.insert(Cord { x: x as i32, y: y as i32, z: 0, w: 0 }); 
                    }
                    _ => { panic!(" building map");  }
                }
            }
        }
        let stay_active = (2, 3);
        let activate = 3;
        let is_part1 = true;
        
        StateMap { map, stay_active, activate, is_part1 }
    }
}

fn read_grid(filename: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(filename).unwrap()
        .trim()
        .split('\n')
        .map(|s| s.as_bytes().to_vec())
        .collect()
}
