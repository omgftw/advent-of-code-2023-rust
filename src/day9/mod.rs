use std::fs;

pub(crate) async fn day9(data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/day9/data/main.txt").unwrap());

    let histories = data.lines().collect::<Vec<&str>>();
    let mut histories_deltas = Vec::new();
    for history in histories {
        let mut history_deltas = Vec::new();
        let history = history.split_whitespace().collect::<Vec<&str>>();
        let mut history = history
            .iter()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // history_deltas.push(history.clone());
        while history.iter().any(|&x| x != 0) {
            history_deltas.push(history.clone());
            history = history
                .windows(2)
                .map(|s| s[1] - s[0])
                .collect::<Vec<i32>>();
        }

        histories_deltas.push(history_deltas);
    }

    let mut histories_total = 0;
    for history_deltas in histories_deltas.clone() {
        for delta in history_deltas.iter().rev() {
            histories_total += delta.last().unwrap();
        }
    }

    let mut histories_total_first = 0;
    for history_deltas in histories_deltas.clone() {
        let mut history_total_first = 0;
        for delta in history_deltas.iter().rev() {
            let first = delta.first().unwrap();
            history_total_first = first - history_total_first;
        }
        histories_total_first += history_total_first;
    }

    (histories_total, histories_total_first)
}