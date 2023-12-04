use std::{collections::HashMap, convert::Infallible, str::FromStr};

#[derive(Debug, Clone)]
struct Card {
    card_no: u32,
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    fn match_count(&self) -> u32 {
        let match_count = self
            .numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count();
        match_count as u32
    }

    fn get_points(&self) -> u32 {
        let match_count = self.match_count();
        if match_count == 0 {
            match_count
        } else {
            2u32.pow(match_count - 1)
        }
    }

    fn get_copies(&self) -> Vec<u32> {
        let mut copies = vec![];
        let match_count = self.match_count();
        for i in 0..match_count {
            let card_no = self.card_no + i + 1;
            copies.push(card_no);
        }
        copies
    }
}

impl FromStr for Card {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (card_no, numbers) = s.split_once(':').unwrap();
        let card_no = card_no
            .strip_prefix("Card")
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap();
        let (winning_numbers, numbers) = numbers.trim().split_once('|').unwrap();
        let winning_numbers = winning_numbers
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        let numbers = numbers
            .split_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        Ok(Self {
            card_no,
            winning_numbers,
            numbers,
        })
    }
}

fn main() {
    let file_name = "input.txt";
    let content = std::fs::read_to_string(file_name).unwrap();
    let cards = content
        .lines()
        .map(|line| Card::from_str(line).unwrap())
        .collect::<Vec<_>>();
    let total_points = cards.iter().map(|c| c.get_points()).sum::<u32>();
    println!("Total points: {}", total_points);

    let mut card_counts = cards
        .iter()
        .map(|c| (c.card_no, 1))
        .collect::<HashMap<_, _>>();
    for card in cards.iter() {
        let copies = card.get_copies();
        let count = card_counts.get(&card.card_no).cloned().unwrap();
        for cp in copies {
            let v = card_counts.get_mut(&cp).unwrap();
            *v += count;
        }
    }

    let total_cards = card_counts.values().sum::<i32>();
    println!("Total cards: {}", total_cards);
}
