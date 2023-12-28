use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use std::fmt::{Display, Formatter, Result};

use regex::Regex;

pub struct Card {
    index: usize,
    winning_numbers: Vec<usize>,
    card_numbers: Vec<usize>,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "Card {}:\nwinning_numbers: {:?}\ncard_numbers: {:?}\n",
            self.index, self.winning_numbers, self.card_numbers
        )
    }
}

pub fn read_card_from_file() -> Vec<Card> {
    let file = File::open("src/inputs/four.txt").unwrap_or_else(|error| {
        panic!("{:?}", error);
    });

    let reader = BufReader::new(file);

    let mut cards: Vec<Card> = vec![];
    for line in reader.lines() {
        let current_line = line.expect("Expect line to be readable");

        let mut split_store: Vec<&str> = current_line.split(":").collect();

        let card_index = extract_card_index(split_store[0]);
        if card_index == 0 {
            continue;
        }

        split_store = split_store[1].trim().split("|").collect();
        let winning_numbers = extract_numbers(split_store[0]);
        let card_numbers = extract_numbers(split_store[1]);

        let card = Card {
            index: card_index,
            winning_numbers,
            card_numbers,
        };

        cards.push(card);
    }

    return cards;
}

// Card index will start from 1, so 0 mean invalid parse result
fn extract_card_index(input: &str) -> usize {
    let regex = Regex::new(r"\d+").expect("Invalid regex");
    let card_index: usize = match regex.find(input) {
        Some(index) => index.as_str().parse().expect("Expect index to be a number"),
        None => 0,
    };

    return card_index;
}

fn extract_numbers(input: &str) -> Vec<usize> {
    let regex = Regex::new(r"\d+").unwrap_or_else(|error| panic!("{:?}", error));

    regex
        .find_iter(input)
        .map(|matched| {
            matched.as_str().parse().unwrap_or_else(|error| {
                panic!(
                    "Unable to parse {} into usize with error {:?}",
                    matched.as_str(),
                    error
                )
            })
        })
        .collect()
}

pub fn calculate_cards_point(cards: &Vec<Card>) -> usize {
    let mut sum = 0;

    for card in cards {
        let mut card_point = 0;
        for card_number in &card.card_numbers {
            if !card.winning_numbers.contains(&card_number) {
                continue;
            }

            if card_point == 0 {
                card_point = 1;
            } else {
                card_point *= 2;
            }
        }
        sum += card_point;
    }

    return sum;
}

pub fn count_duplicate_cards(cards: &Vec<Card>) -> usize {
    let mut counter = 0;
    let mut card_counts: Vec<usize> = vec![1; cards.len() + 10];

    for card in cards {
        let mut match_count = 0;
        for card_number in &card.card_numbers {
            if !card.winning_numbers.contains(&card_number) {
                continue;
            }
            match_count += 1;
        }

        for i in 1..=match_count {
            card_counts[card.index + i] += card_counts[card.index];
        }
        counter += card_counts[card.index]
    }

    return counter;
}
