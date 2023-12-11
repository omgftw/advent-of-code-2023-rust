use std::fs;

struct Race {
    time: i32,
    distance: i32,
    total_possible_times: i32,
}

pub(crate) fn day6(data: Option<String>) -> (i32, i64) {
    let data = data.unwrap_or_else(|| fs::read_to_string("src/data/6.txt").unwrap());
    let data = data.lines().collect::<Vec<&str>>();

    let times = data[0];
    let distances = data[1];
    let times= times.split_whitespace().collect::<Vec<&str>>()[1..].to_vec();
    let distances = distances.split_whitespace().collect::<Vec<&str>>()[1..].to_vec();
    let times = times.iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let distances = distances.iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();

    let mut races = Vec::new();
    for i in 0..times.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
            total_possible_times: 0,
        });
    }

    let mut total = 1;
    for mut race in races {
        for time in 1..race.time {
            if race.distance < (time * (race.time - time)) {
                race.total_possible_times += 1;
            }
        }
        total *= race.total_possible_times;
    }

    let race_time = data[0];
    let race_distance = data[1];
    let race_time = race_time.split(':').collect::<Vec<&str>>()[1].replace(' ', "").parse::<i64>().unwrap();
    let race_distance = race_distance.split(':').collect::<Vec<&str>>()[1].replace(' ', "").parse::<i64>().unwrap();

    let mut total2 = 0;
    for time in 1..race_time {
        if race_distance < time * (race_time - time) {
            total2 += 1;
        }
    }

    // let mut total2 = 0;
    // let half_race_time = race_time / 2;
    // for time in 1..=half_race_time {
    //     let product = time * (race_time - time);
    //     if race_distance < product {
    //         total2 += 1; // Count for the first half
    //         if time != half_race_time || race_time % 2 != 0 {
    //             total2 += 1; // Count for the symmetric second half
    //         }
    //     }
    // }


    (total, total2)
}