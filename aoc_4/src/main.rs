use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::usize;
use std::u8;
extern crate regex;


#[derive(Debug, Clone)]
enum Value {
    Scalar(usize),
    Color { red: u8, green: u8, blue: u8 },
    Text(String),
    Unit { num: usize, unit: String }
}

type Passport = HashMap<String, Value>;
type Passports = Vec<Passport>;

fn main() {
    let passports = read_file("input");
    let result = valid_passports(&passports);
    println!("Valid passports: {}", result);
    
}

fn valid_passports(passports: &Passports) -> usize {
    let mut n_valid = 0;
    for passport in passports {
        n_valid += match passport.len() {
            8 => 1,
            7 if !passport.contains_key("cid") => 1,
            _ => 0
        };
    }
    n_valid
}

fn parse_keyval(entry: &str) -> Passport {
    let mut passport: Passport = HashMap::with_capacity(8);
    let eye_colors: HashSet<&'static str> = 
        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .iter().cloned().collect();

    for field in entry.trim().split(|c: char| c.is_whitespace()) {
        let mut keyval = field.split(":");
        let key = keyval.next().unwrap().to_string();
        let val = keyval.next().unwrap();

        let result = match &key[..] {
           "ecl" if eye_colors.contains(val) => Some(Value::Text(val.to_string())),
           "hcl" => parse_color(&val),
           "hgt" => parse_unit(&val),
           _ => parse_num_fields(&key[..], &val)
        };
        if let Some(result) = result { passport.insert(key, result); }
    }
    passport
}

fn parse_color(hex_str: &str) -> Option<Value> {
    if !(hex_str.starts_with("#") && hex_str.len() == 7) { return None; }
    match (u8::from_str_radix(&hex_str[1..3], 16),
           u8::from_str_radix(&hex_str[3..5], 16),
           u8::from_str_radix(&hex_str[5..], 16)) {
        (Ok(red), Ok(green), Ok(blue)) => Some(Value::Color { red, green, blue }),
        _ => None
    }
}

fn parse_unit(unit_str: &str) -> Option<Value> {
    let re = regex::Regex::new(r"([[:digit:]]*)(cm|in)$").unwrap();
    let cap = match re.captures(unit_str) {
        Some(c) => c,
        None => return None
    };

    let num = match cap[1].parse::<usize>() {
        Ok(x) => x,
        _ => return None
    };
    let unit = cap[2].to_string();
     
    if (unit == "cm" && num >= 150 && num <= 293) || 
        (unit == "in" && num >= 59 && num <= 76) {
            Some(Value::Unit {num, unit} )
    } else {
        None
    }
}

fn parse_num_fields(field: &str, val: &str) -> Option<Value> {
    let num = match val.parse::<usize>() {
        Ok(x) => x,
        _ => return None
    };
    match field {
        "byr" if num >= 1920 && num <= 2002 => Some(Value::Scalar(num)),
        "iyr" if num >= 2010 && num <= 2020 => Some(Value::Scalar(num)),
        "eyr" if num >= 2020 && num <= 2030 => Some(Value::Scalar(num)),
        "pid" if val.len() == 9 => Some(Value::Scalar(num)),
        "cid" => Some(Value::Scalar(num)),
        _ => None
    }
}

fn read_file(filename: &str) -> Passports {
    let buffer = fs::read_to_string(filename).unwrap();
    buffer.split("\n\n")
        .map(parse_keyval)
        .collect()
}
