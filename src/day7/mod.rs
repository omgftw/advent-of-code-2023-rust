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
        m.insert("2", 1);
        m.insert("3", 2);
        m.insert("4", 3);
        m.insert("5", 4);
        m.insert("6", 5);
        m.insert("7", 6);
        m.insert("8", 7);
        m.insert("9", 8);
        m.insert("T", 9);
        m.insert("J", 10);
        m.insert("Q", 11);
        m.insert("K", 12);
        m.insert("A", 13);
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

impl Hand {
    fn partial_cmp(
        &self,
        other: &Self,
        card_order: &HashMap<&str, usize>,
    ) -> Option<std::cmp::Ordering> {
        let self_index = CARD_TYPE_ORDER.get(self.hand_type.as_str()).unwrap();
        let other_index = CARD_TYPE_ORDER.get(other.hand_type.as_str()).unwrap();
        match self_index.cmp(other_index) {
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
            std::cmp::Ordering::Equal => {
                let self_cards = self.cards.chars().collect::<Vec<char>>();
                let other_cards = other.cards.chars().collect::<Vec<char>>();
                for i in 0..self_cards.len() {
                    let self_index = card_order.get(self_cards[i].to_string().as_str()).unwrap();
                    let other_index = card_order.get(other_cards[i].to_string().as_str()).unwrap();
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

fn get_hand_type(cards: &str, jokers_wild: bool) -> String {
    let cards = cards.to_string();

    let mut items = HashMap::new();
    for card in cards.chars() {
        let count = items.entry(card).or_insert(0);
        *count += 1;
    }
    let jokers = &items.entry('J').or_insert(0);
    let jokers = **jokers;
    if jokers_wild {
        items.remove_entry(&'J');
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

    if jokers_wild && jokers > 0 {
        if card_type == "one-pair" {
            if jokers == 1 {
                card_type = "three-of-a-kind";
            } else if jokers == 2 {
                card_type = "four-of-a-kind";
            } else if jokers == 3 {
                card_type = "five-of-a-kind";
            }
        } else if card_type == "two-pair" {
            card_type = "full-house";
        } else if card_type == "three-of-a-kind" {
            if jokers == 1 {
                card_type = "four-of-a-kind";
            } else if jokers == 2 {
                card_type = "five-of-a-kind";
            }
        } else if card_type == "four-of-a-kind" {
            card_type = "five-of-a-kind";
        } else if card_type == "high-card" {
            if jokers == 1 {
                card_type = "one-pair";
            } else if jokers == 2 {
                card_type = "three-of-a-kind";
            } else if jokers == 3 {
                card_type = "four-of-a-kind";
            } else if jokers >= 4 {
                card_type = "five-of-a-kind";
            }
        }
    }

    card_type.to_string()
}

pub(crate) async fn day7(data: Option<String>, jokers_wild: bool) -> i32 {
    let mut card_order = CARD_ORDER.clone();
    if jokers_wild {
        card_order.insert("J", 0);
    }

    let data = data.unwrap_or_else(|| fs::read_to_string("src/day7/data/main.txt").unwrap());

    let mut hands = Vec::new();
    for line in data.lines() {
        let item = line.split(' ').collect::<Vec<&str>>();
        let cards = item[0].to_string();
        let bid = item[1].parse::<i32>().unwrap();
        let hand_type = get_hand_type(&cards, jokers_wild);

        hands.push(Hand {
            cards,
            bid,
            hand_type,
        });
    }
    hands.sort_by(|a, b| a.partial_cmp(b, &card_order).unwrap());

    let mut total = 0;
    for (i, hand) in hands.iter().enumerate() {
        // println!("{:?}", hand);
        total += hand.bid * (i as i32 + 1);
    }

    total
}
