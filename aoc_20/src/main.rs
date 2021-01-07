use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;

fn main() {
    let mut patches = parse_patches("test");
//    for (id, patch) in patches.iter().take(2) {
//        println!("ID: {}", id);
//        for edge in patch.edges.iter() {
//            println!("{}", edge);
//        }
//        println!();
//    }


    let edgemap = build_edgemap(&patches);
    build_image(&mut patches, &edgemap);
}


// Number of pixels per row/column
const SIZE: usize = 10;

static LEFTS: [usize; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
static TOPS: [usize; 8] = [1, 0, 5, 4, 3, 2, 7, 6];
static RIGHTS: [usize; 8] = [2, 3, 0, 1, 6, 7, 4, 5];
static BOTS: [usize; 8] = [3, 2, 7, 6, 1, 0, 5, 4];


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
    data: [Line; SIZE],
    edges: [Line; 8]
}

impl Patch {
    fn new() -> Patch {
        Patch {
            data: [Line::new(); SIZE],
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

    fn get_data(&self, rot: usize) -> [Line; SIZE] {
        let mut ret_data = [Line::new(); SIZE];
        match rot {
            0 => {
                for i in 1..(SIZE - 1) {
                    ret_data[i].bin = (0x7FFF & self.data[i].bin) >> 1;
                }
            }
            _ => panic!("Unknown rotation received")
        }
        ret_data
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

fn build_edgemap(patches: &HashMap<usize, Patch>) -> EdgeMap {
    let mut edge_map = HashMap::new();
    for (id, patch) in patches {
        for edge in &patch.edges {
            let patch_ids = edge_map.entry(*edge).or_insert(Vec::new());
            patch_ids.push(*id);
        }
    }
    edge_map
}


// Finds the four corner-patches of the image. Returns their ids.
// If a line has only one associated patch-id its position must be at
// the border of the image. For a patch to be a corner-patch 4 (2
// normal + 2 flipped) of its edge-lines must have only one associated
// patch-id. 
fn find_corners(edgemap: &EdgeMap) -> Vec<usize> {
    let mut edge_ids: Vec<usize> = Vec::new();
    let mut single_counts: HashMap<usize, usize> = HashMap::new();

    for (_, ids) in edgemap.iter() {
        if ids.len() == 1 {
            let mut id_count = single_counts.entry(ids[0]).or_insert(0);
            *id_count += 1;
            if single_counts[&ids[0]] == 4 {
                edge_ids.push(ids[0]);
                // Quit if 4 corners have been found
                if edge_ids.len() == 4 { break }
            }
        }
    }
    edge_ids
}

fn next_rot_idx(rot: usize, offset: i32) -> usize {
    let wrap = ((rot as i32 - offset) % 4).abs() as usize;
    if rot < 4 { wrap } else { wrap + 4 }
}

fn first_rotation(first: usize, patches: &HashMap<usize, Patch>,
    edge_map: &EdgeMap) -> usize {
    let first_edges = patches[&first].edges;
    // Find left edge 
    for (rot, edge) in first_edges.iter().enumerate() {
        if edge_map[edge].len() == 1 {
            let top = first_edges[next_rot_idx(rot, 1)];
            if edge_map[&top].len() == 1 {
                return rot;
            }
        }
    }
    panic!("First rotation not found");
}

fn build_order(corners: &[usize], side_len: usize, patches: 
    &HashMap<usize, Patch>, edgemap: &EdgeMap) -> Vec<Vec<(usize, usize)>> {

    let mut order: Vec<Vec<(usize, usize)>> = Vec::new();
    let first_rot = first_rotation(1951, patches, edgemap);
    order.push(vec![(1951, first_rot)]);

    let mut i = 0;
    loop { 
        for j in 0..(side_len - 1) {
            let (prev_id, prev_rot) = order[i][j];
            let prev_right = patches[&prev_id].edges[next_rot_idx(prev_rot, 2)];
            println!("{}", prev_right);

            let border_ids = &edgemap[&prev_right];
            let next_id = if border_ids[0] == prev_id { border_ids[1] } 
                else { border_ids[0] };
            let next_rot = patches[&next_id]
                .edges.iter()
                .position(|&line| line == prev_right)
                .unwrap();

            order[i].push((next_id, (next_rot + 4) % 8));
            println!("{:?}", order);
        }
        if i == (side_len - 1) { break }
        let (prev_id, prev_rot) = order[i][0];
        let prev_bot = patches[&prev_id].edges[next_rot_idx(prev_rot, 3)];

        let border_ids = &edgemap[&prev_bot];
        let next_id = if border_ids[0] == prev_id { border_ids[1] } 
            else { border_ids[0] };

        let top_pos = patches[&next_id]
            .edges.iter()
            .position(|&line| line == prev_bot)
            .unwrap();
        let next_rot = (next_rot_idx(top_pos, -1) + 4) % 8;

        order.push(vec![(next_id, next_rot)]);
        i += 1;
    }
    order
}

fn build_image(patches: &HashMap<usize, Patch>, edgemap: &EdgeMap) {
    let side_len = (patches.len() as f64).sqrt() as usize;
    let corners = find_corners(edgemap);
    let p1_answer = corners.iter().fold(1, |acc, &id| id as u64 * acc);
    println!("P1 answer: {}", p1_answer);
    println!("All corners: {:?}", corners);

    let first = 1951;
    let rot = first_rotation(first, patches, edgemap);
    let flipped = if rot < 4 { 0 } else { 4 };

    println!("Rotation of patch at top_left corner: {}", rot);
    println!("Left-edge of patch of that patch: {}", 
        patches[&first].edges[rot]);
    println!("LEFT: {:?}", edgemap[&patches[&first].edges[rot]]);
    println!("TOP: {:?}", edgemap[&patches[&first].edges[next_rot_idx(rot, 1)]]);
    println!("RIGHT: {:?}", edgemap[&patches[&first].edges[next_rot_idx(rot, 2)]]);
    println!("BOT: {:?}", edgemap[&patches[&first].edges[next_rot_idx(rot, 3)]]);
    
    let order = build_order(&corners, side_len, patches, edgemap);
    println!("Order map: \n{:?}", order);


}

fn parse_patch(text: &str, patches: &mut HashMap<usize, Patch>) {
    let mut patch = Patch::new();
    let mut line_iter = text.split('\n');

    let header = line_iter.next().unwrap();
    let id = header[5..9].parse::<usize>().unwrap();
    
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
    patch.edges[0] = patch.get_col(0).flip();
    patch.edges[1] = patch.data[0];
    patch.edges[2] = patch.get_col(SIZE - 1);
    patch.edges[3] = patch.data[SIZE - 1].flip();
    for i in 4..8 {
        patch.edges[i] = patch.edges[i - 4].flip();
    }
    patches.insert(id, patch);
}

fn parse_patches(filename: &str) -> HashMap<usize, Patch> {
    let buffer = fs::read_to_string(filename).unwrap();
    let mut patches: HashMap<usize, Patch> = HashMap::new();
    
    for patch_txt in buffer.trim().split("\n\n") {
        parse_patch(patch_txt, &mut patches);
    }
    patches
}
