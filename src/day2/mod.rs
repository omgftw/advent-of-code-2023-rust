enum Colors {
    Red,
    Blue,
    Green,
    None,
}

struct Cubes {
    quantity: i32,
    color: Colors,
}

struct Set {
    cubes: Vec<Cubes>,
}

pub struct Game {
    id: i32,
    sets: Vec<Set>,
}

fn process_game_line(line: &str) -> Game {
    let parts = line.split(": ").collect::<Vec<&str>>();
    let id = parts[0].split(' ').collect::<Vec<&str>>();
    let id = id[1].parse::<i32>().unwrap();

    let sets_strings = parts[1].split("; ").collect::<Vec<&str>>();
    let mut sets: Vec<Set> = vec![];
    for set in sets_strings {
        let set = Set {
            cubes: process_cube_set(set),
        };
        sets.push(set);
    }
    Game { id, sets }
}

fn process_cube_set(sets: &str) -> Vec<Cubes> {
    let set = sets.split(", ").collect::<Vec<&str>>();
    let mut cubes: Vec<Cubes> = vec![];
    for cube in set {
        let cube = process_cube(cube);
        cubes.push(cube);
    }
    cubes
}

fn process_cube(cube: &str) -> Cubes {
    let parts = cube.split(' ').collect::<Vec<&str>>();
    let quantity = parts[0].parse::<i32>().unwrap();
    let color = parts[1];
    let color = match color {
        "red" => Colors::Red,
        "blue" => Colors::Blue,
        "green" => Colors::Green,
        _ => Colors::None,
    };
    Cubes { quantity, color }
}

pub(crate) fn day2(red: i32, green: i32, blue: i32, data: Option<String>) -> (i32, i32) {
    let data = data.unwrap_or_else(|| std::fs::read_to_string("src/day2/data/main.txt").unwrap());
    let lines = data.lines();

    let mut games: Vec<Game> = vec![];
    for line in lines {
        let game = process_game_line(line);
        games.push(game);
    }

    let mut total = 0;
    let mut min_pow_sum = 0;
    for game in games {
        let mut valid = true;
        // initialize to max
        let mut least_red = 0;
        let mut least_green = 0;
        let mut least_blue = 0;
        for set in game.sets {
            for cube in set.cubes {
                match cube.color {
                    Colors::Red => {
                        if cube.quantity > least_red {
                            least_red = cube.quantity;
                        }
                        if cube.quantity > red {
                            valid = false;
                            // break;
                        }
                    }
                    Colors::Green => {
                        if cube.quantity > least_green {
                            least_green = cube.quantity;
                        }
                        if cube.quantity > green {
                            valid = false;
                            // break;
                        }
                    }
                    Colors::Blue => {
                        if cube.quantity > least_blue {
                            least_blue = cube.quantity;
                        }
                        if cube.quantity > blue {
                            valid = false;
                            // break;
                        }
                    }
                    _ => {
                        valid = false;
                    }
                }
            }
            // if !valid {
            //     break;
            // }
        }
        min_pow_sum += least_red * least_green * least_blue;
        if valid {
            total += game.id;
        }
    }

    (total, min_pow_sum)
}
