use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
pub enum CardHandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl CardHandType {
    pub fn value(&self) -> usize {
        match *self {
            CardHandType::FiveKind => 7,
            CardHandType::FourKind => 6,
            CardHandType::FullHouse => 5,
            CardHandType::ThreeKind => 4,
            CardHandType::TwoPair => 3,
            CardHandType::OnePair => 2,
            CardHandType::HighCard => 1,
        }
    }
}

pub struct CardHand {
    hand: String,
    bid: usize,
    hand_type: CardHandType,
}

impl CardHand {
    pub fn new(hand: &str, bid: usize) -> CardHand {
        CardHand {
            hand: String::from(hand),
            bid,
            hand_type: identify_hand_type(hand),
        }
    }
}

pub fn read_hands_from_file() {
    let file = File::open("src/inputs/seven.txt")
        .unwrap_or_else(|err| panic!("Cannot read day 7 input with error {:?}", err));
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let current_line =
            line.unwrap_or_else(|err| panic!("Cannot read line with error {:?}", err));

        let split_store: Vec<&str> = current_line.split(" ").collect();
        let cards = split_store[0];
        let bid: usize = split_store[1].parse().unwrap_or_else(|err| {
            panic!(
                "Cannot parse {} into an integer with error {:?}",
                split_store[1], err
            )
        });

        let hand = CardHand::new(cards, bid);
        println!("{} is type {:?}", cards, hand.hand_type);
    }
}

fn identify_hand_type(hand: &str) -> CardHandType {
    let mut card_map: HashMap<char, i32> = HashMap::new();

    for character in hand.chars() {
        let stat = card_map.entry(character).or_insert(0);
        *stat += 1;
    }

    let mut three_counter = 0;
    let mut pair_counter = 0;

    for (_, appear_time) in card_map {
        if appear_time == 5 {
            return CardHandType::FiveKind;
        }

        if appear_time == 4 {
            return CardHandType::FourKind;
        }

        if appear_time == 3 {
            three_counter += 1;
            continue;
        }

        if appear_time == 2 {
            pair_counter += 1;
        }
    }

    if three_counter == 1 {
        if pair_counter == 1 {
            return CardHandType::FullHouse;
        }
        return CardHandType::ThreeKind;
    }

    if pair_counter == 2 {
        return CardHandType::TwoPair;
    }

    if pair_counter == 1 {
        return CardHandType::OnePair;
    }

    return CardHandType::HighCard;
}
