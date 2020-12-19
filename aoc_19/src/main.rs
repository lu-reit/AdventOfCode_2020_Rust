use std::fs;
use std::u128;
use std::collections::HashMap;
use std::convert::TryInto;
use regex::Regex;

fn main() {
    let transmission = read_file("input");
    let p1_result = part1(&transmission);
    println!("Part1 result: {}", p1_result);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Message {
    len: u8,
    bin: u128
}

impl Message {
    fn from_str(s: &str) -> Message {
        let len = s.len() as u8;

        let bin_s: String = s.chars() 
            .map(|chr| match chr {
                'a' => '1',
                'b' => '0',
                _ => panic!("Coult not convert message-string: unknown char")
            })
            .collect();

        let bin = u128::from_str_radix(&bin_s, 2)
            .expect("Coult not convert message-string: string too long");
        Message { len, bin }
    }

    fn combine(m1: &Message, m2: &Message) -> Message {
        let len = m1.len + m2.len;
        let bin = (m1.bin << m2.len) | m2.bin;
        Message { len, bin }
    }
}

#[derive(Debug, Clone)]
enum Rule {
    Root(Message),
    Multi(Vec<usize>),
    Or(Vec<usize>),
}

type Rules = Vec<Rule>;

#[derive(Debug, Clone)]
struct Transmission {
    rules: Rules,
    messages: Vec<Message>
}

fn parse_rules(text: &str) -> Rules {
    let re_num = Regex::new("[0-9]+").unwrap();
    let re_letter = Regex::new("a|b").unwrap();
    let mut rules: Vec<(usize, Rule)> = Vec::new();
    for line in text.split('\n') {
        let nums: Vec<usize> = re_num.find_iter(line)
            .map(|digits| digits.as_str().parse().unwrap())
            .collect();
        match line.contains('|') {
            true => {
                rules.push((nums[0], Rule::Or(nums[1..].to_vec())));
            }
            false if nums.len() == 1 => {
                let index = nums[0];
                let message = Message::from_str(
                    re_letter.find_iter(line).next().unwrap().as_str()
                    );
                rules.push((index, Rule::Root(message)));
            }
            false => {
                rules.push((nums[0], Rule::Multi(nums[1..].to_vec())));
            }
            _ => panic!("ol")
        }
    }
    rules.sort_by(|(i, _), (j, _)| i.cmp(j));
    rules.into_iter().map(|(_, rule)| rule).collect()
}

fn combinations(ms_x: Vec<Message>, ms_y: Vec<Message>) -> Vec<Message> {
    let mut combis: Vec<Message> = Vec::with_capacity(ms_x.len() * ms_y.len());

    for m1 in &ms_x {
        for m2 in &ms_y {
            combis.push(Message::combine(&m1, &m2));
        }
    }
    combis
}

fn part1(transmission: &Transmission) -> usize { 
    let patterns = build_patterns(0, &transmission.rules);
    
    let mut sum: usize = 0;
    for message in &transmission.messages {
        for pattern in &patterns {
            if message == pattern { sum += 1 }
        }
    }
    sum
}

fn build_patterns(cur: usize, rules: &Rules) -> Vec<Message> {
    match &rules[cur] {
        Rule::Root(m) => vec![*m],
        Rule::Multi(nodes) if nodes.len() == 1 => {
            build_patterns(nodes[0], rules) 
        }
        Rule::Multi(nodes) => {
            let mut acc = build_patterns(nodes[0], rules);
            for node in &nodes[1..] {
                acc = combinations(acc, build_patterns(*node, rules));
            }
            acc
        }
        Rule::Or(nodes) if nodes.len() == 2 => {
            let mut left = build_patterns(nodes[0], rules);
            let right = build_patterns(nodes[1], rules);
            left.extend(right);
            left
            
        }
        Rule::Or(nodes) => {
            let mut left = combinations(build_patterns(nodes[0], rules),
                                    build_patterns(nodes[1], rules));
            let right = combinations(build_patterns(nodes[2], rules),
                                    build_patterns(nodes[3], rules));
            left.extend(right);
            left
        }
    }
}

 
fn read_file(filename: &str) -> Transmission {
    let buffer = fs::read_to_string(filename).unwrap();
    let mut parts = buffer.trim().split("\n\n");
    let rules = parse_rules(parts.next().unwrap());

    let mut messages: Vec<Message> = Vec::new();
    for line in parts.next().unwrap().split('\n') {
        messages.push(Message::from_str(line));
    }
    Transmission { rules, messages }
}
