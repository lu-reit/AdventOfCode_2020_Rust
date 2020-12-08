use std::fs;

#[derive(Debug, Clone)]
struct Password {
    min: usize,
    max: usize,
    chr: char,
    pass: String
}

fn main() {
    let passwords = read_file("input");
    println!("Part1: {}", part1(&passwords));
    println!("Part2: {}", part2(&passwords));
}

fn part1(passwords: &Vec<Password>) -> usize {
    let mut count: usize = 0;
    for password in passwords {
        let n_matches = password.pass.matches(password.chr).count();
        if n_matches >= password.min && n_matches <= password.max {
            count += 1;
        }
    }
    count
}

fn part2(passwords: &Vec<Password>) -> usize {
    let mut count: usize = 0;
    for password in passwords {
        let chars: Vec<char> = password.pass.chars().collect();

        let lower = chars[password.min - 1] == password.chr;
        let upper = chars[password.max - 1] == password.chr;
        if lower != upper { count += 1 }
    }
    count
}

fn read_file(filename: &str) -> Vec<Password> {
    let buffer = fs::read_to_string(filename).unwrap();

    let mut passwords: Vec<Password> = Vec::new();
    for line in buffer.trim().split("\n") {
        let parts: Vec<&str> = line.split(" ").collect();
        
        let mut minmax = parts[0].split("-");
        let min = minmax.next().unwrap().parse::<usize>().unwrap();
        let max = minmax.next().unwrap().parse::<usize>().unwrap();

        let chr: char = parts[1].chars().next().unwrap();
        let pass = parts[2].to_string();
        
        passwords.push( Password { min, max, chr, pass} );
    }
    passwords
}
