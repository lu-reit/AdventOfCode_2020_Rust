use std::fs;
use std::time::Instant;

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
    while chairmap.variants.len() > 0 {
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
    variants: Vec<Cord>, // List of seats that can still change
    seat_bound: usize,
    is_part1: bool

}

impl ChairMap {
    #[inline]
    fn to_1d(&self, cord: &Cord) -> usize {
        cord.x + cord.y * self.width
    }

    fn get_adjacent(&self, cord: &Cord) -> Vec<Cord> {
        if self.is_part1 { self.get_adjacent_p1(cord) } 
        else { self.get_adjacent_p2(cord) }
    }

    fn get_adjacent_p1(&self, cord: &Cord) -> Vec<Cord> {
        let mut adjacancy: Vec<Cord> = Vec::new();
        for dy in -1..=1 {
            for dx in -1..=1 {
                // Need to use signed integers here otherwise this will overflow
                let (x_n, y_n) = ((cord.x as i32) + dx, (cord.y as i32) + dy);

                // Cant be itself or out of bounds
                if !(dx == 0 && dy == 0) 
                    && ChairMap::in_bounds(x_n, y_n, self.width, self.height) {
                    adjacancy.push( Cord { x: x_n as usize, y: y_n as usize });
                }
            }
        }
        adjacancy
    }

    fn get_adjacent_p2(&self, cord: &Cord) -> Vec<Cord> { 
       let mut adjacent: Vec<Cord> = Vec::new();
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
                       Some(_) => { 
                           adjacent.push(cord);
                           break;
                       }
                   }
               }
           }
       }
       adjacent
    }
     
    fn apply_rules(&self, cord: &Cord) -> Option<bool> {
        let adjacancy = self.get_adjacent(cord);
        let cord = self.to_1d(cord);
        match self.map[cord] {
            Some(occupied) if occupied => { 
                let n_seated = adjacancy.iter()
                    .filter(|&&adj_cord| 
                        self.map[self.to_1d(&adj_cord)] == Some(true))
                    .count();
                if n_seated >= self.seat_bound { Some(false) } else { Some(true) }
            }
            Some(_) => {
                if adjacancy.iter()
                    .any(|&adj_cord| 
                        self.map[self.to_1d(&adj_cord)] == Some(true))
                            { Some(false) } else { Some(true) }
            }
            None => { None }
        }
    }

    fn run_once(&mut self) {
        let mut new_variants: Vec<Cord> = Vec::with_capacity(self.variants.len());
        for &cord in &self.variants {
           if self.apply_rules(&cord) != self.map[self.to_1d(&cord)] { 
               new_variants.push(cord); 
           }
        }
        for cord in &new_variants {
            let cord_1d = self.to_1d(&cord);
            self.map[cord_1d] = match self.map[cord_1d] {
                Some(occupied) => Some(!occupied),
                None => panic!("Empty seat in variants")
           }
        }
        self.variants = new_variants;
    }

    fn build_map(chr_grid: &Vec<Vec<u8>>) -> ChairMap {
        let width = chr_grid[0].len();
        let height = chr_grid.len();
        let n_elem = width * height;
        let mut map: Vec<Option<bool>> = Vec::with_capacity(n_elem);
        let mut variants: Vec<Cord> = Vec::with_capacity(n_elem);

        for (y, chr_row) in chr_grid.iter().enumerate() {
            for (x, chr) in chr_row.iter().enumerate() {
                match chr { 
                    b'.' => { map.push(None); }
                    b'L' => { 
                        // Start with everyone seated
                        map.push(Some(true)); 
                        variants.push( Cord { x, y }); 
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
        println!("");
    }
}

fn read_grid(filename: &str) -> Vec<Vec<u8>> {
    fs::read_to_string(filename).unwrap()
        .trim()
        .split('\n')
        .map(|s| s.as_bytes().to_vec())
        .collect()
}
