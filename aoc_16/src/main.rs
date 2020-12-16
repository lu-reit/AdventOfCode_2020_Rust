use std::fs;
use regex::Regex;
use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Field { 
    id: usize,
    first: (usize, usize),
    second: (usize, usize)
}

impl Field {
    fn new(id: usize, first: (usize, usize), second: (usize, usize)) -> Field {
        Field { id, first, second }
    }
}

type Ticket = Vec<usize>;

#[derive(Debug, Clone)]
struct Notes {
    fields: Vec<Field>,
    own: Ticket,
    others: Vec<Ticket>
}


fn main() {
    let notes = read_notes("input");
    let p1_result = part1(&notes);
    println!("Part 1 result: {}", p1_result);
}

fn in_bound(x: usize, (low, high): (usize, usize)) -> bool {
    x >= low && x <= high
}

fn part1(notes: &Notes) -> usize {
    let mut sum: usize = 0;
    for others in &notes.others {
        'vals: for x in others {
            for field in &notes.fields {
                if in_bound(*x, field.first) || in_bound(*x, field.second) {
                    continue 'vals
                }
            }
            sum += x;
        }
    }
    sum
}

fn read_fields(text: &str, re: &Regex) -> Vec<Field> {
    let mut fields: Vec<Field> = Vec::new();

    // Use a id instead of a String to identify fields
    for (id, entry) in text.split('\n').enumerate() {
        // Match the numbers, parse them, and get a iterator over pairs
        let mut r_iter = re.find_iter(entry)
            .map(|m| m.as_str().parse::<usize>().unwrap())
            .tuples::<(usize, usize)>(); // From itertools

        let first = r_iter.next().unwrap();
        let second = r_iter.next().unwrap();
        fields.push(Field::new(id, first, second));
    }
    fields
}

fn read_notes(filename: &str) -> Notes {
    let buffer = fs::read_to_string(filename).unwrap();
    // Split the text into three parts: the fields, santas ticket and other tickets
    let mut parts = buffer.trim().split("\n\n");
    let re = Regex::new(r"\d+").unwrap();

    let fields = read_fields(parts.next().unwrap(), &re);

    let own = re.find_iter(parts.next().unwrap())
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .collect();

    let mut others: Vec<Ticket> = Vec::new();
    // Iterate over each ticket, skip the header
    for ticket_text in parts.next().unwrap().split('\n').skip(1) {
        let ticket = re.find_iter(ticket_text)
            .map(|m| m.as_str().parse::<usize>().unwrap())
            .collect();
        others.push(ticket);
    }
    Notes { fields, own, others }
}
    
