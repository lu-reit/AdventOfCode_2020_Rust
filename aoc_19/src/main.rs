use std::fs;
use std::u128;
use regex::Regex;
use std::time::Instant;

static MAX_DEPTH: usize = 3;

fn main() {
    let mut transmission = read_file("input");
    
    let p1_timer = Instant::now();
    let p1_result = part1(&transmission);
    let p1_time = p1_timer.elapsed();
    println!("Part1 result: {}", p1_result);
    println!("Part1 time: {:?}", p1_time);
    let p2_timer = Instant::now();
    let p2_result = part2(&transmission);
    let p2_time = p2_timer.elapsed();
    println!("Part2 result: {}", p2_result);
    println!("Part2 time: {:?}", p2_time);
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

    fn is_substr(&self, other: &Message) -> bool {
        let ones = (1 << other.len) - 1;
        for i in 0..=(self.len - other.len) {
            let test_bin = ((ones << i) & self.bin) >> i;
            if test_bin == other.bin {
                return true;
            }
        }
        return false;
    }
}

#[derive(Debug, Clone)]
enum Rule {
    Root(Message),
    Multi(Vec<usize>),
    Or((Vec<usize>, Vec<usize>)),
}

type Rules = Vec<Rule>;

#[derive(Debug, Clone)]
struct Transmission {
    rules: Rules,
    messages: Vec<Message>
}

fn part1(transmission: &Transmission) -> usize { 
    let mut sum: usize = 0;
    for message in transmission.messages.iter() {
        let (_, exists, _) = match_rules(0, &message, &transmission.rules);
        if exists {
            sum += 1;
        }
    }
    sum
}

fn part2(transmission: &Transmission) -> usize {
    let mut sum: usize = 0;
    for message in transmission.messages.iter() {
        if up_to_max_depth(&message, &transmission.rules) {
            sum += 1;
        }
    }
    sum
}

fn up_to_max_depth(message: &Message, rules: &Rules) -> bool {
    let (_, _, pat42) = match_rules(42, message, rules);
    let (_, _, pat31) = match_rules(31, message, rules);
    true
}




fn combinations(acc: &[Message], to_push: &[Message]) -> Vec<Message> {
    let mut combis = Vec::new();
    for m1 in acc.iter() {
        for m2 in to_push.iter() {
            combis.push(Message::combine(&m1, &m2));
        }
    }
    combis
}

fn test_patterns(patterns: &[Message], target: &Message) -> (bool, bool, Vec<Message>) {
    let mut substr_exists = false; 
    let mut matches: Vec<Message> = Vec::new();
    for m in patterns.into_iter() {
        if m == target {
            return (true, true, vec![*m]);
        } else if m.len > target.len {
            continue;
        } else { 
            if target.is_substr(&m) {
                substr_exists = true;
                matches.push(*m);
            }
        }
    }
    if substr_exists {
        (false, false, matches)
    } else {
        (true, false, matches)
    }
}

fn recurse(nodes: &[usize], target: &Message, rules: &Rules) ->
 (bool, bool, Vec<Message>) {
     if nodes.len() == 1 {
         match_rules(nodes[0],  target, rules)
     } else if nodes.len() == 2 {
         recurse_twice(nodes, target, rules)
     } else {
         recurse_all(nodes, target, rules)
     }
}

fn recurse_twice(nodes: &[usize], target: &Message, rules: &Rules)
    -> (bool, bool, Vec<Message>) {
    let (quit, found, left) = match_rules(nodes[0], target, rules);
    if quit {
       return (quit, found, left);
    } 

    let (quit, found, right) = match_rules(nodes[1], target, rules);
    if quit {
        return (quit, found, right);
    } 

    let new_comb = combinations(&left, &right);
    test_patterns(&new_comb, target)
}

fn recurse_all(nodes: &[usize], target: &Message, rules: &Rules) 
    -> (bool, bool, Vec<Message>) {
    let mut all: Vec<Vec<Message>> = Vec::new();
     
    for i in 0..nodes.len() {
        let (quit, found, patterns) = match_rules(nodes[i], target, rules);
        if quit {
           return (quit, found, patterns);
        } 
        all.push(patterns);
    }
     
    let mut acc: Vec<Message> = all[0].clone();
    for i in 1..nodes.len() {
        acc = combinations(&acc, &all[i].clone());
    }

    test_patterns(&acc, target)
}

fn match_rules(cur: usize, target: &Message, rules: &Rules) 
    -> (bool, bool, Vec<Message>) {
    match &rules[cur] {
        Rule::Root(m) => {
            return (false, false, vec![*m]);
        }
        Rule::Multi(nodes) => {
            recurse(&nodes, target, rules)
        }
        Rule::Or(nodes) => {
            let (lquit, lfound, mut lacc) = recurse(&nodes.0, target, rules);
            if lquit && lfound { return (lquit, lfound, lacc) }
            let (rquit, rfound, racc) = recurse(&nodes.1, target, rules);
            if rquit && rfound { (rquit, rfound, racc) }
            else if lquit && rquit { (true, false, lacc) }
            else if lquit && !rquit { (false, false, racc) }
            else if !lquit && rquit { (false, false, lacc) }
            else {  
                lacc.extend(racc);
                (false, false, lacc)
            }
        }
    }
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
                let len = (nums.len() - 1) / 2 + 1;
                rules.push((nums[0], Rule::Or((nums[1..len].to_vec(),
                                               nums[len..].to_vec()))
                ));
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
        }
    }
    rules.sort_by(|(i, _), (j, _)| i.cmp(j));
    rules.into_iter().map(|(_, rule)| rule).collect()
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
