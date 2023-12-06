use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Range {
    source_start_idx: u64,
    destination_start_idx: u64,
    len: u64,
}

#[derive(Debug)]
struct SourceDestinationMap {
    source: String,
    destination: String,
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Input {
    seeds: Vec<u64>,
    seed_to_soil_map: SourceDestinationMap,
    soil_to_fertilizer_map: SourceDestinationMap,
    fertilizer_to_water_map: SourceDestinationMap,
    water_to_light_map: SourceDestinationMap,
    light_to_temprature_map: SourceDestinationMap,
    temperature_to_humidity_map: SourceDestinationMap,
    humidity_to_location_map: SourceDestinationMap,
}

fn read_ranges<'a>(it: &mut impl Iterator<Item = &'a str>) -> Vec<Range> {
    let mut line = it.next().unwrap();
    let mut result = Vec::new();
    while !line.is_empty() {
        let range_vals: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        result.push(Range {
            source_start_idx: (&range_vals)[1],
            destination_start_idx: (&range_vals)[0],
            len: (&range_vals)[2],
        });
        line = it.next().unwrap_or("");
    }
    result
}

fn parse_input(input_str: &str) -> Input {
    let mut input_it = input_str.lines();
    let seeds_line = input_it.next().unwrap();
    let seeds: Vec<u64> = seeds_line[6..]
        .trim()
        .split(' ')
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect();
    assert!(input_it.next() == Some(""));
    assert!(input_it.next() == Some("seed-to-soil map:"));
    let seed_to_soil_map = SourceDestinationMap {
        source: String::from("seed"),
        destination: String::from("soil"),
        ranges: read_ranges(&mut input_it),
    };
    assert!(input_it.next() == Some("soil-to-fertilizer map:"));
    let soil_to_fertilizer_map = SourceDestinationMap {
        source: String::from("soil"),
        destination: String::from("fertilizer"),
        ranges: read_ranges(&mut input_it),
    };
    assert!(input_it.next() == Some("fertilizer-to-water map:"));
    let fertilizer_to_water_map = SourceDestinationMap {
        source: String::from("fertilizer"),
        destination: String::from("water"),
        ranges: read_ranges(&mut input_it),
    };
    assert!(input_it.next() == Some("water-to-light map:"));
    let water_to_light_map = SourceDestinationMap {
        source: String::from("water"),
        destination: String::from("light"),
        ranges: read_ranges(&mut input_it),
    };
    assert!(input_it.next() == Some("light-to-temperature map:"));
    let light_to_temprature_map = SourceDestinationMap {
        source: String::from("light"),
        destination: String::from("temperature"),
        ranges: read_ranges(&mut input_it),
    };
    assert!(input_it.next() == Some("temperature-to-humidity map:"));
    let temprature_to_humidity_map = SourceDestinationMap {
        source: String::from("temperature"),
        destination: String::from("humidity"),
        ranges: read_ranges(&mut input_it),
    };
    assert!(input_it.next() == Some("humidity-to-location map:"));
    let humidity_to_location_map = SourceDestinationMap {
        source: String::from("humidity"),
        destination: String::from("location"),
        ranges: read_ranges(&mut input_it),
    };

    Input {
        seeds,
        seed_to_soil_map,
        soil_to_fertilizer_map,
        fertilizer_to_water_map,
        water_to_light_map,
        light_to_temprature_map,
        temperature_to_humidity_map: temprature_to_humidity_map,
        humidity_to_location_map,
    }
}

fn get_mapping(value: u64, current_map: &SourceDestinationMap) -> u64 {
    for range in &current_map.ranges {
        if value >= range.source_start_idx && value < (range.source_start_idx + range.len) {
            let result = (value - range.source_start_idx) + range.destination_start_idx;
            return result;
        }
    }
    value
}

fn get_mappings(values: HashSet<u64>, current_map: &SourceDestinationMap) -> HashSet<u64> {
    let result = values
        .into_iter()
        .map(|v| get_mapping(v, current_map))
        .collect();
    return result;
}

fn calculate(input: Input) -> u64 {
    let maps = HashMap::from([
        ("seed".to_string(), &input.seed_to_soil_map),
        ("soil".to_string(), &input.soil_to_fertilizer_map),
        ("fertilizer".to_string(), &input.fertilizer_to_water_map),
        ("water".to_string(), &input.water_to_light_map),
        ("light".to_string(), &input.light_to_temprature_map),
        ("temperature".to_string(), &input.temperature_to_humidity_map),
        ("humidity".to_string(), &input.humidity_to_location_map),
    ]);

    let mut values: HashSet<u64> = HashSet::from_iter(input.seeds);
    let mut value_name = String::from("seed");
    while value_name != "location" {
        let current_map = maps.get(&value_name).unwrap();
        values = get_mappings(values, &current_map);
        value_name = current_map.destination.clone();
    }
    values.into_iter().min().unwrap()
}

pub fn run() {
    let input_str = include_str!("../input.txt");
    let input = parse_input(input_str);
    let result = calculate(input);
    println!("Result for day05a: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let sample_input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";
        let result = calculate(parse_input(sample_input));
        assert_eq!(result, 35);
    }
}
