use std::fs;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;


fn main() {
    let mut foods = read_allergens("input");
    let p1_result = part1(&foods);
    println!("Result of part1: {}", p1_result);
}
 
type BString = Vec<u8>;
type FoodMap = HashMap<BString, Vec<HashSet<BString>>>;

fn b_to_s(bstring: &[u8]) -> String {
    String::from_utf8_lossy(bstring).to_string()
}

struct Foods {
    total: usize,
    ingreds: HashMap<BString, usize>,
    fmap: HashMap<BString, HashSet<BString>>,
}

impl Foods {
    fn new() -> Foods {
        Foods { 
            total: 0,
            ingreds: HashMap::new(),
            fmap: HashMap::new(),
        }
    }
}

fn part1(foods: &Foods) -> usize {
    let mut known: HashSet<BString> = HashSet::new();
    let mut known_total = 0;

    for (_, ingred_set) in foods.fmap.iter() {
        for ingred in ingred_set.iter() {
            if !known.contains(ingred) {
                known.insert(ingred.clone());
                known_total += foods.ingreds[ingred];
            }
        }
    }
    foods.total - known_total
}

fn read_allergens(filename: &str) -> Foods {
    let buffer = read_ascii_trim(filename);
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
            let entry = foods.ingreds.entry(i_vec).or_insert(0);
            *entry += 1;
            cur_ingreds.insert(ingred.to_vec());
        }
        foods.total += cur_ingreds.len();

        for allergen in allergen_txt.split(|&c| c == b',') {
            let a_vec = if allergen[0] == b' ' { allergen[1..].to_vec() }
                else { allergen.to_vec() };
            match foods.fmap.get_mut(&a_vec) {
                Some(set) => {
                    let intersection = set.intersection(&cur_ingreds)
                        .cloned()
                        .collect();
                    *set = intersection;
                }
                None => { 
                    foods.fmap.insert(a_vec, cur_ingreds.clone());
                }
            }
        }
    }
    for (ingred, count) in foods.ingreds.iter() {
        println!("Ingredient: {};\t count: {}", b_to_s(&ingred), count);
    }
    for (all, intersec) in foods.fmap.iter() {
        print!("Allergen {}: ", b_to_s(&all));
        for ingred in intersec {
            print!("{}, ", b_to_s(&ingred));
        }
        println!();
    }
    println!("Total: {}", foods.total);
    foods
}

fn read_ascii_trim(filename: &str) -> BString {
    let mut file = File::open(filename).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    if buffer[buffer.len() - 1] == b'\n' { buffer.pop(); }
    buffer
}
