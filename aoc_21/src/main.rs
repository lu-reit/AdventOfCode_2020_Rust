use std::fs;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;


fn main() {
    read_allergens("test");
}
 
type BString = Vec<u8>;
type FoodMap = HashMap<BString, Vec<HashSet<BString>>>;

fn b_to_s(bstring: &[u8]) -> String {
    String::from_utf8_lossy(bstring).to_string()
}

struct Foods {
    ingreds: HashMap<BString, usize>,
    fmap: FoodMap,
}

impl Foods {
    fn new() -> Foods {
        Foods { 
            ingreds: HashMap::new(),
            fmap: HashMap::new(),
        }
    }
}

fn read_allergens(filename: &str) {
    let mut buffer = read_ascii_trim(filename);
    println!("{}", b_to_s(&buffer));
    println!("{}", buffer.len());
    let mut foods = Foods::new();

    for line in buffer.split(|&c| c == b'\n') {
        let mut parts = line.split(|&c| c == b'(');
        let mut ingred_txt = parts.next().unwrap();
        ingred_txt = &ingred_txt[..(ingred_txt.len() - 1)];

        let mut allergen_txt = parts.next().unwrap();
        allergen_txt = &allergen_txt[9..(allergen_txt.len() - 1)];

        let mut cur_ingreds: HashSet<BString> = HashSet::new();
        for ingred in ingred_txt.split(|&c| c.is_ascii_whitespace()) {
            let i_vec = ingred.to_vec();
            let mut entry = foods.ingreds.entry(i_vec).or_insert(0);
            *entry += 1;
            cur_ingreds.insert(ingred.to_vec());
        }

        for allergen in allergen_txt.split(|&c| c.is_ascii_whitespace()) {
            let a_vec = allergen.to_vec();
            let mut entry = foods.fmap.entry(a_vec).or_insert(Vec::new());
            entry.push(cur_ingreds.clone());
        }
    }
    for (ingred, count) in foods.ingreds.iter() {
        println!("Ingredient: {};\t count: {}", b_to_s(&ingred), count);
    }
}

fn read_ascii_trim(filename: &str) -> BString {
    let mut file = File::open(filename).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    if buffer[buffer.len() - 1] == b'\n' { buffer.pop(); }
    buffer
}
