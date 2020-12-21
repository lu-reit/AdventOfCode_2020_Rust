use std::fs;
use std::collections::HashMap;

// Number of pixels per row/column
const SIZE: usize = 10;

// A Line of SIZE pixels; either a row or a column 
// represented by an integer. The (SIZE - 1) bit
// represents the first pixel in a row/column
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Line {
    bin: u16
}

impl Line {

    //Reverse a row by x-oring with a bit-string of ones
    #[inline]
    fn reverse(&self) -> Line {
        let ones = (1 << SIZE) - 1;
        Line { bin: self.bin ^ ones }
    }
    
    #[inline]
    fn swap_nth(&mut self, nth: usize) {
        let nth_one = 1 << (SIZE - 1 - nth);
        self.bin = self.bin ^ nth_one;
    }
}

// A patch of the resulting image. Patches are always square.
// Patches can be rotated and flipped. The current orientation 
// is encoded by a single u8 (rot) as follows:
// 0: no rottation no flip
// 1: rotate 90 no flip
// 2: rotate 180 no flip
// 3: rotate 270 no flip
// 4 - 7: like 0 - 3 but flipped
// All rotations are clockwise
struct Patch {
    id: usize,
    data: [u16; SIZE],
    rot: u16, // Lets avoid indroducing another integer type
    edges: [u16; 8]
}

type EdgeMap = HashMap<u16, Vec<usize>>;

fn main() {
    parse_images("input");
}


fn parse_images(filename: &str) {
    let buffer = fs::read_to_string(filename).unwrap();
    let mut edge_map: EdgeMap = HashMap::new();
    let images: Vec<Patch> = Vec::new();
    
    let mut n = 0;
    for image in buffer.trim().split("\n\n") {
        n += 1;
    }
    println!("N = {}",n );
}
