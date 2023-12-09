use std::fs;

#[derive(Clone)]
struct Map {
    source: i64,
    dest: i64,
    range: i64,
}

// struct Data {
//     seed_to_soil: Map,
//     soil_to_fertilizer: Map,
//     fertilizer_to_water: Map,
//     water_to_light: Map,
//     light_to_temperature: Map,
//     temperature_to_humidity: Map,
//     humidity_to_location: Map,
// }

fn parse_data(data: &str) -> (Vec<i64>, Vec<Vec<Map>>) {
    let mut maps: Vec<Vec<Map>> = vec![];
    let data = data.split("\n\n").collect::<Vec<&str>>();

    let seeds = data[0];
    let seeds = seeds.split(' ').collect::<Vec<&str>>();
    let seeds = seeds[1..].to_vec();
    let seeds = seeds.iter().map(|s| s.parse::<i64>().unwrap()).collect::<Vec<i64>>();
    for (i, mapping_type) in data.iter().enumerate() {
        let mut processed_mapping_type = Vec::new();
        if i == 0 {
            continue;
        }
        let mappings = mapping_type.split('\n').collect::<Vec<&str>>();
        let mappings = mappings[1..].to_vec();

        for mapping in mappings {
            let mapping = mapping.split(' ').collect::<Vec<&str>>();
            let dest = mapping[0].parse::<i64>().unwrap();
            let source = mapping[1].parse::<i64>().unwrap();
            let range = mapping[2].parse::<i64>().unwrap();
            processed_mapping_type.push(Map {
                source,
                dest,
                range,
            });
        }
        maps.push(processed_mapping_type);
    }

    (seeds, maps)
}

pub(crate) fn day5(data: Option<String>) -> (i64, i64) {
    let data = match data {
        Some(data) => data,
        None => fs::read_to_string("src/data/5.txt").unwrap(),
    };

    let (mut seeds, maps) = parse_data(&data);
    for map in maps {
        let mut new_seeds = Vec::new();
        for seed in seeds.iter() {
            let mut handled = false;
            for map in &map {
                if seed >= &map.source && seed <= &(map.source + map.range) {
                    new_seeds.push(map.dest + (seed - map.source));
                    handled = true;
                    break;
                }
            }
            if !handled {
                new_seeds.push(*seed);
            }
        }
        seeds = new_seeds;
    }

    let mut lowest_seed = seeds[0];
    for seed in seeds {
        if seed < lowest_seed {
            lowest_seed = seed;
        }
    }
    (lowest_seed, 0)
}