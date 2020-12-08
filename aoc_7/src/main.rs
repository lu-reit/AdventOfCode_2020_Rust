use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;
extern crate regex;

type BagMap = HashMap<String, Vec<(String, usize)>>;

fn main() {
    let bag_rules = read_file("input");
    let bag_map = build_bag_map(&bag_rules);
    
    let p1_timer = Instant::now();
    let p1_result = part1(&bag_map);
    let p1_time = p1_timer.elapsed();
    println!("Result of part1: {}", p1_result);
    println!("Time of part1: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&bag_map);
    let p2_time = p2_timer.elapsed();
    println!("Result of part2: {}", p2_result);
    println!("Time of part2: {:?}", p2_time);
}

fn part2(bag_map: &BagMap) -> usize {
    p2_recurse(&"shiny gold".to_string(), bag_map)
}

fn p2_recurse(cur_bag: &String, bag_map: &BagMap) -> usize  {
    if bag_map[cur_bag].len() == 0 {
        0
    }
    else {
        let mut sub_bags = 0;
        for bag in &bag_map[cur_bag] {
            sub_bags += p2_recurse(&bag.0, &bag_map) * bag.1 + bag.1;
        }
        sub_bags
    }
}

fn part1(bag_map: &BagMap) -> usize {
    let mut has_gold: HashSet<&String> = HashSet::with_capacity(bag_map.len());
    let mut no_gold: HashSet<&String> = HashSet::with_capacity(bag_map.len());

    for bag in bag_map {
        if !has_gold.contains(bag.0) || !no_gold.contains(bag.0) { 
            p1_recurse(bag.0, &mut has_gold, &mut no_gold, &bag_map); 
        }
    }
    has_gold.len()
}


fn p1_recurse<'a>(cur_bag: &'a String, has_gold: &mut HashSet<&'a String>, 
    no_gold: &mut HashSet<&'a String>, bag_map: &'a BagMap) -> bool {
    if cur_bag == "shiny gold" { true }
    else {
        for bag in &bag_map[cur_bag] {
            if no_gold.contains(&bag.0) { continue; }
            else if has_gold.contains(&bag.0) || p1_recurse(&bag.0, has_gold, no_gold, bag_map) {
                has_gold.insert(&cur_bag);
                return true;
            }
        }
        no_gold.insert(&cur_bag);
        false
    }
}

fn build_bag_map(bag_rules: &Vec<String>) -> BagMap {
    let mut bag_map: BagMap = BagMap::with_capacity(bag_rules.len());
    let re = regex::Regex::new(r" bags contain | bag[s, .]*").unwrap();

    for rule in bag_rules {
        let mut children: Vec<(String, usize)> = Vec::new();

        let mut parts = re.split(rule);
        let parent = parts.next().unwrap();
        for child in parts {
            if child.is_empty() || child.starts_with("no") { break; }
            else {
                children.push((
                        child[2..].to_string(),
                        child[..1].parse::<usize>().unwrap()));
            }
        }
        bag_map.insert(parent.to_string(), children);
    }
    bag_map
}

fn read_file(filename: &str) -> Vec<String> {
    fs::read_to_string(filename).unwrap()
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect()
}
