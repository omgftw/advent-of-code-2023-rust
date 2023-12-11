use std::fs;

#[derive(Debug, Clone)]
struct Card {
    count: i32,
    winning_count: i32,
}

pub(crate) fn day4(data: Option<String>) -> (i32, i32) {
    let data = match data {
        Some(data) => data,
        None => fs::read_to_string("src/data/4.txt").unwrap(),
    };

    let mut total_score = 0;
    let mut all_cards_count = 0;
    let mut all_cards = vec![];

    for line in data.lines() {
        let parts = line.split(':').collect::<Vec<&str>>();
        let numbers = parts[1].split('|').collect::<Vec<&str>>();
        let winning_numbers = numbers[0].split_whitespace().collect::<Vec<&str>>();
        let your_numbers = numbers[1].split_whitespace().collect::<Vec<&str>>();

        let mut total = 0;
        let mut winning_count = 0;
        for num in your_numbers {
            if winning_numbers.contains(&num) {
                winning_count += 1;
                if total == 0 {
                    total = 1;
                } else {
                    total *= 2;
                }
            }
        }
        total_score += total;

        all_cards.push(Card {
            count: 1,
            winning_count,
        });
    }

    for i in 0..all_cards.len() {
        let card = &mut all_cards[i];
        let card = card.clone();
        all_cards_count += card.count;
        for x in 0..card.winning_count as usize {
            let other = &mut all_cards[x + i + 1];
            other.count += card.count;
        }
    }

    (total_score, all_cards_count)
}
