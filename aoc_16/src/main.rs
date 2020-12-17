use std::fs;
use regex::Regex;
use itertools::Itertools;
use std::time::Instant;

#[derive(Debug, Clone)]
struct Field { 
    first: (usize, usize),
    second: (usize, usize)
}

impl Field {
    fn new(first: (usize, usize), second: (usize, usize)) -> Field {
        Field {first, second }
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
    let filename = "input";
    let notes = read_notes(filename);
     
    let timer = Instant::now();
    let (p1_result, mut occurence) = find_faulty(&notes);
    let p2_result = part2(&occurence, &notes);
    let elapsed = timer.elapsed();
    println!("Part 1 result: {}", p1_result);
    println!("Part 2 result: {}", p2_result);
    println!("Time both: {:?}", elapsed);
}

#[inline]
fn in_bound(x: usize, (low, high): (usize, usize)) -> bool {
    x >= low && x <= high
}

fn update_bool_mat(update: &[Vec<bool>], old: &mut [Vec<bool>]) {
    for i in 0..old.len() {
        for j in 0..old.len() {
            old[i][j] = old[i][j] && update[i][j];
        }
    }
}

fn print_rot_mat(mat: &[(usize, usize, Vec<bool>)]) {
    for i in 0..mat.len() {
        print!("{:02} [ ", mat[i].0);
        for j in 0..mat.len() {
            match mat[i].2[j] {
                true => { print!("T ") }
                false => { print!("F ") }
            }
        }
        println!("]");
    }
    println!();
}

// Finds faulty tickets and computes the answer for part 1
fn find_faulty(notes: &Notes) -> (usize, Vec<Vec<bool>>) {
    let len = notes.fields.len();
    let mut global: Vec<Vec<bool>> = vec![vec![true; len]; len]; 

    let mut sum: usize = 0;
    'row: for other in notes.others.iter() {
        let mut update: Vec<Vec<bool>> = vec![vec![true; len]; len]; 
             
        for (i, &x) in other.iter().enumerate() {
            let mut is_faulty = true;

            for (j, field) in notes.fields.iter().enumerate() {
                let is_first = in_bound(x, field.first);
                let is_second = in_bound(x, field.second);
                let any = is_first || is_second;
                if any {
                    is_faulty = false;
                }
                update[j][i] = any;
            }
            if is_faulty {
                sum += x;
                continue 'row;
            }
        }
        update_bool_mat(&update, &mut global);
    }
    (sum, global)
}

fn part2(occurence: &[Vec<bool>], notes: &Notes) -> u64 {
    let len = occurence.len();
    let mut rot: Vec<(usize, usize, Vec<bool>)> = Vec::new();

    for i in 0..len {
        let mut true_sum = 0;
        let mut col: Vec<bool> = Vec::new();

        for j in 0..len {
            if occurence[j][i] { true_sum += 1 }
            col.push(occurence[j][i]);
        }
        rot.push((i, true_sum, col));
    }
    rot.sort_by(|(_, sum1, _), (_, sum2, _)| sum1.cmp(sum2));

    let mut order: Vec<(usize, usize)> = Vec::new();
    for i in 0..len {
        if rot[0].2[i] == true {
            order.push((rot[0].0, i));
        }
    }

    let mut acc = rot[0].2.clone();
    for i in 1..len {
        let col_i = rot[i].0;
        let mut col = &mut rot[i].2;
        for j in 0..len {
             col[j] = col[j] != acc[j];
             if col[j] == true { 
                 order.push((col_i, j));
                 acc[j] = col[j] || acc[j];
             }
        }
    }

    let mut prod: u64 = 1;
    for (col_i, field_i) in order {
        if field_i < 6 {
            prod *= notes.own[col_i] as u64;
        }
    }
    prod
}

fn read_fields(text: &str, re: &Regex) -> Vec<Field> {
    let mut fields: Vec<Field> = Vec::new();

    // Use a id instead of a String to identify fields
    for entry in text.split('\n') {

        // Match the numbers, parse them, and get a iterator over pairs
        let mut r_iter = re.find_iter(entry)
            .map(|m| m.as_str().parse::<usize>().unwrap())
            .tuples::<(usize, usize)>(); // From itertools

        let first = r_iter.next().unwrap();
        let second = r_iter.next().unwrap();
        fields.push(Field::new(first, second));
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
    
