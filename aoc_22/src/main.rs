use std::fs;
use std::collections::VecDeque;

fn main() {
    let mut cards = parse_game("input");
    let winner = play_game(&mut cards);
    print_cards(winner, &cards[winner]);
    let p1_result = part1(winner, &cards);
    println!("Part 1 result: {}", p1_result);

}

fn part1(winner: usize, cards: &[VecDeque<u32>; 2]) -> u64 {
    let n_cards = cards[winner].len();
    cards[winner].iter()
        .zip((1..=n_cards).rev())
        .fold(0, |acc, (&card, nth)| acc + (card as u64 * nth as u64))
}

fn play_game(cards: &mut [VecDeque<u32>; 2]) -> usize {
    loop {
        match (cards[0].is_empty(), cards[1].is_empty()) {
            (true, false) => { return 1 }
            (false, true) => { return 0 }
            (true, true) => { panic!("Both stacks are empty") }
            _ => {}
        }
        let card1 = cards[0].pop_front().unwrap();
        let card2 = cards[1].pop_front().unwrap();
        if card1 > card2 { 
            cards[0].push_back(card1);
            cards[0].push_back(card2);
        } else {
            cards[1].push_back(card2);
            cards[1].push_back(card1);
        }
    }
}
        
fn print_cards(player: usize, cards: &VecDeque<u32>) {
    println!("Player {};", player + 1);
    for card in cards.iter() {
        println!("{}", card);
    }
}

fn parse_game(filename: &str) -> [VecDeque<u32>; 2] {
    let buffer = fs::read_to_string(filename).unwrap();
    let mut parts = buffer.trim().split("\n\n");

    [parse_player_cards(parts.next().unwrap()), 
     parse_player_cards(parts.next().unwrap())]
}

fn parse_player_cards(cards_txt: &str) -> VecDeque<u32> {
    cards_txt.split('\n').skip(1) 
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}
