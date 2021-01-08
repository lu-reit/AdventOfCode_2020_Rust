use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;


fn main() {
    read_allergens("test");
}
 
struct Foods<'a> {
    ingreds: Vec<String>,
    allergens: Vec<String>,
    fmap: HashMap<&'a str, Vec<HashSet<&'a str>>>,
    unknowns: HashSet<&'a str>
}

impl<'a> Foods<'a> {
    fn new() -> Foods<'a> {
        Foods { 
            ingreds: Vec::new(),
            allergens: Vec::new(),
            fmap: HashMap::new(),
            unknowns: HashSet::new(),
        }
    }
}

fn read_allergens(filename: &str) {
    let mut buffer = fs::read_to_string(filename).unwrap();
    //let mut foodmap: FoodMap = HashMap::new();
    let mut ingreds: HashSet<String> = HashSet::new();
    for line in buffer.trim().split('\n') {
        
    }
    println!("{}", buffer);
}

