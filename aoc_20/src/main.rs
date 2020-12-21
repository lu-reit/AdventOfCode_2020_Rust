use std::fs;
use std::collections::HashMap;

// Number of pixels per row/column
const SIZE: usize = 10;

struct Image {
    id: usize,
    data: Vec<Vec<bool>>,
    edges: [[bool; SIZE]; 8]
}


fn main() {
    println!("Hello, world!");
}

fn parse_images(filename: &str) {
    let buffer = fs::read_to_string(filename).unwrap();
    let mut edge_map: HashMap<[bool; 10], usize> = HashMap::new();
    let mut width = 0;
    let mut height = 0;


    for image in buffer.trim().split("\n\n") {
       let mut line_iter = image.split('\n');

       let id_line = line_iter.next().unwrap();
       let id = &id_line[5..9].parse::<usize>();

       let mut left: [bool; SIZE] = [false; SIZE];
       let mut right: [bool; SIZE] = [false; SIZE]; 
    }
}
