use std::fs;
use std::time::Instant;
use std::collections::HashSet;

fn main() {
    let mut statemap1 = StateMap::build_map(&read_grid("input"));
    let mut statemap2 = statemap1.clone();
    statemap2.is_part1 = false;

    let p1_timer = Instant::now();
    run_statemap(6, &mut statemap1);
    let p1_result = statemap1.map.len();
    let p1_time = p1_timer.elapsed();
    println!("Result part1: {}", p1_result);
    println!("Time part1: {:?}", p1_time);

    let p2_timer = Instant::now();
    run_statemap(6, &mut statemap2);
    let p2_result = statemap2.map.len();
    let p2_time = p2_timer.elapsed();
    println!("Result part2: {}", p2_result);
    println!("Time part2: {:?}", p2_time);
}

fn run_statemap(times: usize, statemap: &mut StateMap) {
    for _ in 0..times {
        statemap.run_once();
    }
}

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

    fn num_adjacent(&self, cord: &Cord) -> usize {
        if self.is_part1 {
            self.num_adjacent3(cord)
        } else {
            self.num_adjacent4(cord)
        }
    }

    fn num_adjacent3(&self, cord: &Cord) -> usize {
        let mut adjacent: usize = 0;
        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                for dz in -1i32..=1 {
                    if dx.abs() + dy.abs() + dz.abs() > 0  {
                        let neighbour = Cord { x: cord.x + dx, y: cord.y + dy, 
                                               z: cord.z + dz, w: 0 };
                        if self.at_cord(&neighbour) { adjacent += 1; }
                    }
                }
            }
        }
        adjacent 
    }

    fn num_adjacent4(&self, cord: &Cord) -> usize {
        let mut adjacent: usize = 0;
        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                for dz in -1i32..=1 {
                    for dw in -1i32..=1 {
                        if dx.abs() + dy.abs() + dz.abs() + dw.abs() > 0  {
                            let neighbour = Cord { x: cord.x + dx, y: cord.y + dy, 
                                                   z: cord.z + dz, w: cord.w + dw };
                            if self.at_cord(&neighbour) { adjacent += 1; }
                        }
                    }
                }
            }
        }
        adjacent 
    }
    
    fn test_active(&self, cord: &Cord) -> bool {
        let n_adjacent = self.num_adjacent(cord);
        n_adjacent == self.stay_active.0 || n_adjacent == self.stay_active.1
    }

    fn test_inactive(&self, cord: &Cord) -> bool {
        let n_adjacent = self.num_adjacent(cord);
        n_adjacent == self.activate
    }

    fn test_neighbours(&self, cord: &Cord, update: &mut Vec<Cord>) {
        if self.is_part1 { 
            self.test_neighbours3(cord, update);
        } else {
            self.test_neighbours4(cord, update);
        }
    }

    fn test_neighbours3(&self, cord: &Cord, update: &mut Vec<Cord>) {
        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                for dz in -1i32..=1 {
                    if dx.abs() + dy.abs() + dz.abs() > 0 {
                        let neighbour = Cord { x: cord.x + dx, y: cord.y + dy, 
                                          z: cord.z + dz, w: 0 };
                        if self.at_cord(&neighbour) { continue } 
                        if self.test_inactive(&neighbour) {
                            update.push(neighbour);
                        }
                    }
                }
            }
        }
    }

    fn test_neighbours4(&self, cord: &Cord, update: &mut Vec<Cord>) {
        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                for dz in -1i32..=1 {
                    for dw in -1i32..=1 {
                        if dx.abs() + dy.abs() + dz.abs() + dw.abs() > 0 {
                            let neighbour = Cord { x: cord.x + dx, y: cord.y + dy, 
                                              z: cord.z + dz, w: cord.w + dw };
                            if self.at_cord(&neighbour) { continue } 
                            if self.test_inactive(&neighbour) {
                                update.push(neighbour);
                            }
                        }
                    }
                }
            }
        }
    }

    fn apply_rules(&self, cord: &Cord, update: &mut Vec<Cord>) {
        self.test_neighbours(cord, update);
        if self.test_active(&cord) { update.push(*cord) }
    }

    fn run_once(&mut self) {
        let mut update: Vec<Cord> = Vec::new();
        for cord in self.map.iter() {
            self.apply_rules(&cord, &mut update);
        }
        self.map = update.into_iter().collect();
    }

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
