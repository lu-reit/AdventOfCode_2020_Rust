use std::fs;
use std::time::Instant;
use std::collections::VecDeque;

fn main() {
    let mut chairmap1 = ChairMap::build_map(&read_grid("input"));
    let mut chairmap2 = chairmap1.clone();
    chairmap2.is_part1 = false;
    chairmap2.seat_bound = 5;

    let p1_timer = Instant::now();
    let p1_result = run_chairmap(&mut chairmap1);
    let p1_time = p1_timer.elapsed();
    println!("Result part1: {}", p1_result);
    println!("Time part1: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = run_chairmap(&mut chairmap2);
    let p2_time = p2_timer.elapsed();
    println!("Result part2: {}", p2_result);
    println!("Time part2: {:?}", p2_time);
}

fn run_chairmap(chairmap: &mut ChairMap) -> usize {
    while !chairmap.variants.is_empty() {
        chairmap.run_once();
    }
    chairmap.map.iter().filter(|&&cord| cord == Some(true)).count()
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Cord {
    x: usize,
    y: usize
}

#[derive(Debug, Clone)]
struct ChairMap {
    width: usize,
    height: usize,
    // None -> no seat; Some(false) -> empty seat, Some(true) -> occupied seat 
    map: Vec<Option<bool>>, 
    variants: VecDeque<Cord>, // List of seats that can still change
    seat_bound: usize, 
    is_part1: bool

}

impl ChairMap {
    fn to_1d(&self, cord: &Cord) -> usize {
        cord.x + cord.y * self.width
    }

    fn get_adjacent(&self, cord: &Cord) -> usize {
        if self.is_part1 { self.get_adjacent_p1(cord) } 
        else { self.get_adjacent_p2(cord) }
    }

    fn get_adjacent_p1(&self, cord: &Cord) -> usize {
        let mut adjacent: usize = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 { continue }

                // Need to use signed integers here otherwise this will overflow
                let (x_n, y_n) = ((cord.x as i32) + dx, (cord.y as i32) + dy);
                if !ChairMap::in_bounds(x_n, y_n, self.width, self.height) { continue}

                let cord = Cord { x: x_n as usize, y: y_n as usize };
                if self.map[self.to_1d(&cord)] == Some(true) { adjacent += 1; }
            }
        }
        adjacent 
    }

    fn get_adjacent_p2(&self, cord: &Cord) -> usize { 
       let mut adjacent = 0;
       let c_x = cord.x as i32;
       let c_y = cord.y as i32;

       for dir_y in -1..=1 {
           for dir_x in -1..=1 {
               if dir_x == 0 && dir_y == 0 { continue }
                
               let mut x_i = c_x + dir_x;
               let mut y_i = c_y + dir_y;
               loop { 
                   if !ChairMap::in_bounds(x_i, y_i, self.width, self.height) { break }
                   let cord = Cord { x: x_i as usize, y: y_i as usize };
                   match self.map[self.to_1d(&cord)] {
                       None => {
                            x_i += dir_x;
                            y_i += dir_y;
                       }
                       Some(occupied) if occupied => { 
                           adjacent += 1;
                           if adjacent == self.seat_bound { return adjacent }
                           break;
                       }
                       Some(_) => { break }
                   }
               }
           }
       }
       adjacent
    }
     
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

    #[inline]
    fn run_once(&mut self) {
        let var_len = self.variants.len();
        for _ in 0..var_len {
            let cord = match self.variants.pop_front() {
                Some(c) => c,
                _ => return
            };
            let cord_1d = self.to_1d(&cord);
            let seat = self.map[cord_1d];
            if self.apply_rules(&cord) != seat {
                self.variants.push_back(cord);
            }
        }

        for cord in self.variants.iter() {
            let cord_i = self.to_1d(&cord);
            self.map[cord_i] = match self.map[cord_i] {
                Some(occupied) => Some(!occupied),
                _ => panic!("Encountered an empty seat in main loop")
            };
        }
    }

    fn build_map(chr_grid: &[Vec<u8>]) -> ChairMap {
        let width = chr_grid[0].len();
        let height = chr_grid.len();
        let n_elem = width * height;
        let mut map: Vec<Option<bool>> = Vec::with_capacity(n_elem);
        let mut variants: VecDeque<Cord> = VecDeque::with_capacity(n_elem);

        for (y, chr_row) in chr_grid.iter().enumerate() {
            for (x, chr) in chr_row.iter().enumerate() {
                match chr { 
                    b'.' => { map.push(None); }
                    b'L' => { 
                        // Start with everyone seated
                        map.push(Some(true)); 
                        variants.push_back( Cord { x, y }); 
                    }
                    _ => { panic!("Failed building map"); 
                    }
                }
            }
        }
        let seat_bound = 4;
        let is_part1 = true;
        
        ChairMap { width, height, map, variants, seat_bound, is_part1 }
    }

    // Check whether a position is still inside the grid
    #[inline(always)]
    fn in_bounds(x: i32, y: i32, width: usize, height: usize) -> bool {
        x >= 0 && x < (width as i32) && y >= 0 && y < (height as i32)
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
        println!();
    }
}

fn read_grid(filename: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(filename).unwrap()
        .trim()
        .split('\n')
        .map(|s| s.as_bytes().to_vec())
        .collect()
}
