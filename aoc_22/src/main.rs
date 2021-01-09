use std::fs;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::time::Instant;

fn main() {
    let mut cards = parse_game("input");
    let mut cards2 = cards.clone();


    let p1_timer = Instant::now();
    let winner = play_game(&mut cards);
    let p1_result = count_points(winner, &cards);
    let p1_time = p1_timer.elapsed();
    println!("Part 1 result: {}", p1_result);
    println!("Part 1 time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let winner = play_recursive(&mut cards2);
    let p2_result = count_points(winner, &cards2);
    let p2_time = p1_timer.elapsed();
    println!("Part 2 result: {}", p2_result);
    println!("Part 2 time: {:?}", p2_time);

}

fn play_recursive(cards: &mut [VecDeque<usize>; 2]) -> usize {
    // Optimisation: If one player holds the highest-valued card
    // the other player can only win if the recursive rule triggers. However
    // if the value is greater than the combined length of both
    // players card-stack recursion will never trigger. However player
    // 0 can still attain victory through the infinite recursion rule.
    // So we can apply this optimisation only if player 0 holds the
    // highest card.
    let max0 = cards[0].iter().max().unwrap();
    let max1 = cards[1].iter().max().unwrap();
    if max0 > max1 && max0 > &(cards[0].len() + cards[1].len()) { 
        return 0 
    } 
    let mut history0: HashSet<VecDeque<usize>> = HashSet::new();
    let mut history1: HashSet<VecDeque<usize>> = HashSet::new();

    loop {
        match (cards[0].is_empty(), cards[1].is_empty()) {
            (true, false) => { return 1 }
            (false, true) => { return 0 }
            (true, true) => { panic!("Both stacks are empty") }
            _ => {}
        }
        // Check for infinite recursion
        if history0.contains(&cards[0]) || history1.contains(&cards[1]) {
            return 0; 
        }

        update_history(&mut history0, &mut history1, &cards);
        let card0 = cards[0].pop_front().unwrap();
        let card1 = cards[1].pop_front().unwrap();

        // Determine the winner either through a sub-game or if that
        // is not applicable by the value of their cards
        let winner = if card0 <= cards[0].len() && card1 <= cards[1].len() {
            let mut new_cards: [VecDeque<usize>; 2] = [
                cards[0].iter().take(card0 as usize).copied().collect(),
                cards[1].iter().take(card1 as usize).copied().collect()
            ];
            play_recursive(&mut new_cards)
        } else if card0 > card1 { 0 } else { 1 };

        if winner == 0 { 
            cards[0].push_back(card0);
            cards[0].push_back(card1);
        } else {
            cards[1].push_back(card1);
            cards[1].push_back(card0);
        }
    }
}

fn update_history(hist0: &mut HashSet<VecDeque<usize>>, 
                  hist1: &mut HashSet<VecDeque<usize>>,
                  cards: &[VecDeque<usize>; 2]) {
    hist0.insert(cards[0].iter().cloned().collect());
    hist1.insert(cards[1].iter().cloned().collect());
}

fn count_points(winner: usize, cards: &[VecDeque<usize>; 2]) -> usize {
    let n_cards = cards[winner].len();
    cards[winner].iter()
        .zip((1..=n_cards).rev())
        .fold(0, |acc, (&card, nth)| acc + card * nth as usize)
}

fn play_game(cards: &mut [VecDeque<usize>; 2]) -> usize {
    loop {
        match (cards[0].is_empty(), cards[1].is_empty()) {
            (true, false) => { return 1 }
            (false, true) => { return 0 }
            (true, true) => { panic!("Both queues are empty") }
            _ => {}
        }
        let card0 = cards[0].pop_front().unwrap();
        let card1 = cards[1].pop_front().unwrap();
        if card0 > card1 { 
            cards[0].push_back(card0);
            cards[0].push_back(card1);
        } else {
            cards[1].push_back(card1);
            cards[1].push_back(card0);
        }
    }
}
        
fn print_cards(player: usize, cards: &VecDeque<usize>) {
    print!("Player {}: ", player + 1);
    for card in cards.iter() {
        print!("{}, ", card);
    }
    println!();
}

fn parse_game(filename: &str) -> [VecDeque<usize>; 2] {
    let buffer = fs::read_to_string(filename).unwrap();
    let mut parts = buffer.trim().split("\n\n");

    [parse_player_cards(parts.next().unwrap()), 
     parse_player_cards(parts.next().unwrap())]
}

fn parse_player_cards(cards_txt: &str) -> VecDeque<usize> {
    cards_txt.split('\n').skip(1) 
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}
