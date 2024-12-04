use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use core::cmp::Ordering;
use regex::Regex;
use core::fmt;

fn main() {
    let re_line_parse: regex::Regex = Regex::new(r"[0-9A-Z]+").unwrap();
    if let Ok(file_iter) = read_lines("input_7.txt") {
        let mut all_hands: Vec<Hand> = Vec::new();
        for line in file_iter {
            if let Ok(text) = line {
                let mut parsed_text = re_line_parse.find_iter(&text);
                let hand_cards = parsed_text.next().unwrap();
                let new_bid = parsed_text.next().unwrap().as_str().parse::<u64>().unwrap();
                let new_cards = get_cards(hand_cards.as_str().to_string());
                let new_strength = get_strength(new_cards);
                //println!("Hand {:?} has strength {}", new_cards, new_strength);
                let new_hand = Hand{cards: new_cards, bid: new_bid, strength: new_strength};
                all_hands.push(new_hand);
            }
        }
        all_hands.sort();
        let mut total = 0;
        for (i, hand) in all_hands.into_iter().enumerate() {
            total += hand.bid * (i+1) as u64;
        }
        println!("Total: {}", total);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_cards(new_hand: String) -> [u64; 5] {
    let mut result: [u64; 5] = [0; 5];
    for (i, character) in new_hand.chars().enumerate() {
        if let Some(card) = character.to_digit(10) {
            result[i] = card as u64;
            continue;
        } else {
            result[i] = match character {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 0,
                'T' => 10,
                _ => 0,
            }
        }
    }
    return result;
}

fn get_strength(card_group: [u64; 5]) -> Strength {
    let mut card_count_max = [0; 14];
    let mut num_jokers = 0;
    for card in card_group {
        if card == 0 {
            num_jokers += 1;
        } else {
            card_count_max[(card - 1) as usize] += 1;
        }
    }
    card_count_max.sort();
    card_count_max.reverse();
    let max_count = card_count_max[0];
    let min_count = card_count_max[1];
    //println!("Max count: {}, min count: {}", max_count, min_count);
    match max_count+num_jokers {
        5 => {return Strength::FiveKind},
        4 => {return Strength::FourKind},
        3 => {if min_count.clone() == 2 { return Strength::FullHouse;} else { return Strength::ThreeKind;}},
        2 => {
            let mut num_pairs = 0;
            for count in card_count_max {
                if count == 2 {
                    num_pairs += 1;
                }
            }
            if num_pairs == 2 { return Strength::TwoPair;} else {return Strength::OnePair;}
        }
        _ => {return Strength::HighCard},
    }
}

struct Hand {
    cards: [u64; 5],
    bid: u64,
    strength: Strength,
}

impl PartialOrd for Hand
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(&other));
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.strength.cmp(&other.strength) == Ordering::Equal {
            return self.cards.cmp(&other.cards);
        } else {
            return self.strength.cmp(&other.strength);
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.strength != other.strength {
            return false;
        } else {
           return self.cards.eq(&other.cards);
        }
    }
}

impl Eq for Hand {}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum Strength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl fmt::Display for Strength {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match *self {
            Strength::HighCard => write!(f, "High Card"),
            Strength::OnePair => write!(f, "One Pair"),
            Strength::TwoPair => write!(f, "Two Pair"),
            Strength::ThreeKind => write!(f, "Three Kind"),
            Strength::FullHouse => write!(f, "Full House"),
            Strength::FourKind => write!(f, "Four Kind"),
            Strength::FiveKind => write!(f, "Five Kind"),
        }
    }
}