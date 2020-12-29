use std::fs;
use std::collections::HashMap;
use std::fmt;

fn main() {
    let mut patches = parse_patches("input");
    let edgemap = build_edgemap(&patches);
    build_image(&patches, &edgemap);
}


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
    fn new() -> Line {
        Line { bin: 0 }
    }

    // Reverse/flip a line.
    // Currently this just copies the bits one by one
    #[inline]
    fn flip(&self) -> Line {
        let mut flipped = 0;
        let mut bin_old = self.bin;
        for i in 0..SIZE {
            flipped = (flipped | (bin_old & 1)) << 1;
            bin_old >>= 1;
        }
        flipped >>= 1;
        Line { bin: flipped }
    }
    
    // Swap the nth-bit in a bit-string. Note that bit-strings
    // here are zero-indexed from the left. The left-most bit
    // is not the most-significant bit but the (SIZE - 1) bit. So
    // if SIZE = 5 and x = 0001 1111, then x.swap_nth(0) = 0000 1111
    #[inline]
    fn swap_nth(&mut self, nth: usize) {
        let nth_one = 1 << (SIZE - 1 - nth);
        self.bin = self.bin ^ nth_one;
    }
    
    #[inline] 
    fn get_nth(&self, nth: usize) -> u16 {
        let nth_one = 1 << (SIZE - 1 - nth);
        self.bin & nth_one
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output: String = String::new();
        for i in 0..SIZE {
            if self.get_nth(i) > 0 {
                output.push_str("#"); 
            } else {
                output.push_str("."); 
            }
        }
        write!(f, "{}", output)
    }
}


// A patch of the resulting image. Patches are always square.
// Patches can be rotated and flipped. The current orientation 
// is encoded by a single u16 (rot) as follows:
// 0: no rotation no flip (this is also the starting rotation)
// 1: rotate 90 no flip
// 2: rotate 180 no flip
// 3: rotate 270 no flip
// 4 - 7: like 0 - 3 but flipped along the x-axis
// All rotations are clockwise
struct Patch {
    id: usize,
    data: [Line; SIZE],
    rot: u16, 
    edges: [Line; 8]
}

impl Patch {
    fn new() -> Patch {
        Patch {
            id: 0,
            data: [Line::new(); SIZE],
            rot: 0,
            edges: [Line::new(); 8]
        }
    }

    fn get_col(&self, nth: usize) -> Line {
        let mut col = Line::new();
        for i in 0..SIZE {
            col.bin |= ((self.data[i].get_nth(nth) != 0) as u16) << (SIZE - 1 - i);
        }
        col
    }
}

impl fmt::Display for Patch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..SIZE {
            writeln!(f, "{}", self.data[i]).expect("Failed to print patch");
        }
        Ok(())
    }
}

type EdgeMap = HashMap<Line, Vec<usize>>;

fn build_edgemap(patches: &[Patch]) -> EdgeMap {
    let mut edge_map = HashMap::new();
    for patch in patches {
        for edge in &patch.edges {
            let patch_ids = edge_map.entry(*edge).or_insert(Vec::new());
            patch_ids.push(patch.id);
        }
    }
    edge_map
}

fn build_image(patches: &[Patch], edgemap: &EdgeMap) {
    let side_len = (patches.len() as f64).sqrt() as usize;
    let mut order: Vec<Vec<(usize, u8)>> = vec![vec![(0,0); side_len]; side_len];
    println!("Side len: {}", side_len);

}

fn parse_patch(text: &str) -> Patch {
    let mut patch = Patch::new();
    let mut line_iter = text.split('\n');

    let header = line_iter.next().unwrap();
    patch.id = header[5..9].parse::<usize>().unwrap();
    
    for (i, line) in line_iter.enumerate() {
        for (j, chr) in line.chars().enumerate() {
            match chr {
                '#' => {
                    patch.data[i].swap_nth(j);
                }
                '.' => { }
                _ => panic!("Unknown character encountered during patch parsing")
            }
        }
    }
    // Set the edges (left,top,right,bottom) and their flipped versions
    patch.edges[0] = patch.get_col(0);
    patch.edges[1] = patch.data[0];
    patch.edges[2] = patch.get_col(SIZE - 1);
    patch.edges[3] = patch.data[SIZE - 1];
    for i in 4..8 {
        patch.edges[i] = patch.edges[i - 4].flip();
    }
    patch
}

fn parse_patches(filename: &str) -> Vec<Patch> {
    let buffer = fs::read_to_string(filename).unwrap();
    let mut patches: Vec<Patch> = Vec::new();
    
    for patch_txt in buffer.trim().split("\n\n") {
        patches.push(parse_patch(patch_txt));
    }
    patches
}
