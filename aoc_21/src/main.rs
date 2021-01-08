use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

fn main() {
    // Doesnt make sense here to time the parts seperatly,
    // so i just time everything
    let timer = Instant::now();
    let mut foods = read_allergens("input");
    solve_foods(&mut foods);
    let p1_result = part1(&foods);
    let p2_result = part2(&foods);
    let elapsed = timer.elapsed(); 
    println!("Result of part1: {}", p1_result);
    println!("Result of part2: {}", p2_result);
    println!("Total time: {:?}", elapsed);
}
 
struct Foods {
    total: usize,
    ingreds: HashMap<String, usize>,
    fmap: HashMap<String, HashSet<String>>,
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

fn part2(foods: &Foods) -> String {
    let mut pairs: Vec<(&str, &str)> = Vec::new();
    for (allergen, ingred_set) in foods.fmap.iter() {
        pairs.push((&allergen[..], &ingred_set.iter().next().unwrap()[..]));
    }
    pairs.sort_by(|(a1, _), (a2, _)| a1.cmp(a2));
    let mut buffer = String::from(pairs[0].1);
    for (_, ingred) in pairs.iter().skip(1) {
        buffer = format!("{},{}", buffer, ingred);
    }
    buffer
}
    
fn part1(foods: &Foods) -> usize {
    let mut known_total = 0;
    for (_, ingred_set) in foods.fmap.iter() {
        let ingred = ingred_set.iter().next().unwrap();
        known_total += foods.ingreds[ingred];
    }
    foods.total - known_total
}

fn solve_foods(foods: &mut Foods) {
    let mut known: HashSet<String> = HashSet::new();

    loop { 
        let mut all_found = true;
        for (_, ingred_set) in foods.fmap.iter_mut() {
            if ingred_set.len() == 1 {
                let single = ingred_set.iter().next().unwrap();
                known.insert(single.clone());
            } else {
                let diff = ingred_set.difference(&known)
                    .cloned()
                    .collect();
                *ingred_set = diff;
                all_found = false;
            }
        }
        if all_found { break }
    }
}

fn read_allergens(filename: &str) -> Foods {
    let buffer = fs::read_to_string(filename).unwrap();
    let mut foods = Foods::new();

    for line in buffer.trim().split('\n') {
        let mut parts = line.split(" (");
        let ingred_txt = parts.next().unwrap();

        let mut allergen_txt = parts.next().unwrap();
        allergen_txt = &allergen_txt[9..(allergen_txt.len() - 1)];

        let mut cur_ingreds: HashSet<String> = HashSet::new();
        for ingred in ingred_txt.split(' ') {
            let i_vec = ingred.to_string();
            let entry = foods.ingreds.entry(i_vec).or_insert(0);
            *entry += 1;
            cur_ingreds.insert(ingred.to_string());
        }
        foods.total += cur_ingreds.len();

        for allergen in allergen_txt.split(", ") {
            let a_vec = allergen.to_string();
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
    foods
}
