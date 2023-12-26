use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: i32,
    hand_type: String,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

lazy_static! {
    static ref CARD_ORDER: HashMap<&'static str, usize> = {
        let mut m = HashMap::new();
        m.insert("2", 0);
        m.insert("3", 1);
        m.insert("4", 2);
        m.insert("5", 3);
        m.insert("6", 4);
        m.insert("7", 5);
        m.insert("8", 6);
        m.insert("9", 7);
        m.insert("T", 8);
        m.insert("J", 9);
        m.insert("Q", 10);
        m.insert("K", 11);
        m.insert("A", 12);
        m
    };
}

lazy_static! {
    static ref CARD_TYPE_ORDER: HashMap<&'static str, usize> = {
        let mut m = HashMap::new();
        m.insert("high-card", 0);
        m.insert("one-pair", 1);
        m.insert("two-pair", 2);
        m.insert("three-of-a-kind", 3);
        m.insert("full-house", 4);
        m.insert("four-of-a-kind", 5);
        m.insert("five-of-a-kind", 6);
        m
    };
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_index = CARD_TYPE_ORDER.get(self.hand_type.as_str()).unwrap();
        let other_index = CARD_TYPE_ORDER.get(other.hand_type.as_str()).unwrap();
        match self_index.cmp(other_index) {
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
            std::cmp::Ordering::Equal => {
                let self_cards = self.cards.chars().collect::<Vec<char>>();
                let other_cards = other.cards.chars().collect::<Vec<char>>();
                for i in 0..self_cards.len() {
                    let self_index = CARD_ORDER.get(self_cards[i].to_string().as_str()).unwrap();
                    let other_index = CARD_ORDER.get(other_cards[i].to_string().as_str()).unwrap();
                    match self_index.cmp(other_index) {
                        std::cmp::Ordering::Less => return Some(std::cmp::Ordering::Less),
                        std::cmp::Ordering::Greater => return Some(std::cmp::Ordering::Greater),
                        std::cmp::Ordering::Equal => continue,
                    }
                }
                Some(std::cmp::Ordering::Equal)
            }
        }
    }
}

fn get_hand_type(cards: &str) -> String {
    let mut items = HashMap::new();
    for card in cards.chars() {
        let count = items.entry(card).or_insert(0);
        *count += 1;
    }
    let mut card_type = "high-card";
    for (_, count) in items {
        if count == 2 && card_type == "three-of-a-kind" {
            card_type = "full-house";
        } else if count == 2 && card_type == "one-pair" {
            card_type = "two-pair";
        } else if count == 2 {
            card_type = "one-pair";
        } else if count == 3 && card_type == "one-pair" {
            card_type = "full-house";
        } else if count == 3 {
            card_type = "three-of-a-kind";
        } else if count == 4 {
            card_type = "four-of-a-kind";
        } else if count == 5 {
            card_type = "five-of-a-kind";
        }
    }
    card_type.to_string()
}

pub(crate) async fn day7(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/data/7.txt").unwrap());

    let mut hands = Vec::new();
    for line in data.lines() {
        let item = line.split(' ').collect::<Vec<&str>>();
        let cards = item[0].to_string();
        let bid = item[1].parse::<i32>().unwrap();
        let hand_type = get_hand_type(&cards);

        hands.push(Hand {
            cards,
            bid,
            hand_type,
        });
    }
    hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        // println!("{:?}", hand);
        total += hand.bid * (i as i32 + 1);
    }

    (total, 0)
}