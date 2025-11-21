use utils;
use regex::Regex;
use std::cmp;


fn main() {
    let raw_input = utils::lines_from_file("day05");
    let p1: u64 = calc_p1(&raw_input);
    let p2: u64 = calc_p2(&raw_input);
    println!("p1: {}", p1);
    println!("p2: {}", p2);
}


fn calc_p1(raw_input: &Vec<String>) -> u64 {
    let mut p1: u64 = 0;
    let seeds = raw_input[0]
        .split(":").collect::<Vec<&str>>()[1].trim()
        .split(" ").collect::<Vec<&str>>()
        .iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>()
    ;

    let mut current_group = "None";
    let mut seed_to_soil_map: Vec<Vec<u64>> = Vec::new();
    let mut soil_to_fertilizer_map: Vec<Vec<u64>> = Vec::new();
    let mut fertilizer_to_water_map: Vec<Vec<u64>> = Vec::new();
    let mut water_to_light_map: Vec<Vec<u64>> = Vec::new();
    let mut light_to_temperature_map: Vec<Vec<u64>> = Vec::new();
    let mut temperature_to_humidity_map: Vec<Vec<u64>> = Vec::new();
    let mut humidity_to_location_map: Vec<Vec<u64>> = Vec::new();
    for row in raw_input {
        if row == "" {
            continue;
        }
        if row == "seed-to-soil map:" {
            current_group = "seed-to-soil map";
            continue;
        }
        if row == "soil-to-fertilizer map:" {
            current_group = "soil-to-fertilizer map";
            continue;
        }
        if row == "fertilizer-to-water map:" {
            current_group = "fertilizer-to-water map";
            continue;
        }
        if row == "water-to-light map:" {
            current_group = "water-to-light map";
            continue;
        }
        if row == "light-to-temperature map:" {
            current_group = "light-to-temperature map";
            continue;
        }
        if row == "temperature-to-humidity map:" {
            current_group = "temperature-to-humidity map";
            continue;
        }
        if row == "humidity-to-location map:" {
            current_group = "humidity-to-location map";
            continue;
        }
        if current_group == "seed-to-soil map" {
            seed_to_soil_map.push(
                row.split(" ").collect::<Vec<&str>>()
                .iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>()
            );
        }
        if current_group == "soil-to-fertilizer map" {
            soil_to_fertilizer_map.push(
                row.split(" ").collect::<Vec<&str>>()
                .iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>()
            );
        }
        if current_group == "fertilizer-to-water map" {
            fertilizer_to_water_map.push(
                row.split(" ").collect::<Vec<&str>>()
                .iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>()
            );
        }
        if current_group == "water-to-light map" {
            water_to_light_map.push(
                row.split(" ").collect::<Vec<&str>>()
                .iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>()
            );
        }
        if current_group == "light-to-temperature map" {
            light_to_temperature_map.push(
                row.split(" ").collect::<Vec<&str>>()
                .iter().map(|x| x.parse
                ::<u64>().unwrap()).collect::<Vec<u64>>()
            );
        }
        if current_group == "temperature-to-humidity map" {
            temperature_to_humidity_map.push(
                row.split(" ").collect::<Vec<&str>>()
                .iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>()
            );
        }
        if current_group == "humidity-to-location map" {
            humidity_to_location_map.push(
                row.split(" ").collect::<Vec<&str>>()
                .iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>()
            );
        }
    }

    for seed in seeds {
        let mut soil: u64 = 0;
        let mut fertilizer: u64 = 0;
        let mut water: u64 = 0;
        let mut light: u64 = 0;
        let mut temperature: u64 = 0;
        let mut humidity: u64 = 0;
        let mut location: u64 = 0;

        for row in &seed_to_soil_map {
            if seed >= row[1] && seed < row[1] + row[2] {
                soil = row[0] + (seed - row[1]);
                break;
            }
        }
        if soil == 0 {
            soil = seed;
        }

        for row in &soil_to_fertilizer_map {
            if soil >= row[1] && soil < row[1] + row[2] {
                fertilizer = row[0] + (soil - row[1]);
                break;
            }
        }
        if fertilizer == 0 {
            fertilizer = soil;
        }

        for row in &fertilizer_to_water_map {
            if fertilizer >= row[1] && fertilizer < row[1] + row[2] {
                water = row[0] + (fertilizer - row[1]);
                break;
            }
        }
        if water == 0 {
            water = fertilizer;
        }

        for row in &water_to_light_map {
            if water >= row[1] && water < row[1] + row[2] {
                light = row[0] + (water - row[1]);
                break;
            }
        }
        if light == 0 {
            light = water;
        }

        for row in &light_to_temperature_map {
            if light >= row[1] && light < row[1] + row[2] {
                temperature = row[0] + (light - row[1]);
                break;
            }
        }
        if temperature == 0 {
            temperature = light;
        }

        for row in &temperature_to_humidity_map {
            if temperature >= row[1] && temperature < row[1] + row[2] {
                humidity = row[0] + (temperature - row[1]);
                break;
            }
        }
        if humidity == 0 {
            humidity = temperature;
        }

        for row in &humidity_to_location_map {
            if humidity >= row[1] && humidity < row[1] + row[2] {
                location = row[0] + (humidity - row[1]);
                break;
            }
        }
        if location == 0 {
            location = humidity;
        }

        if p1 == 0 {
            p1 = location;
        } else {
            p1 = cmp::min(p1, location);
        }
    }
    p1
}

// fn calc_p2(raw_input: &Vec<String>) -> u64 {
//     let mut p2: u64 = 0;
//     let seed_ranges = raw_input[0]
//         .split(":").collect::<Vec<&str>>()[1].trim()
//         .split(" ").collect::<Vec<&str>>()
//         .iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>()
//     ;
//     let mut seeds: Vec<u64> = Vec::new();
//     for seed_range in seed_ranges.chunks(2) {
//         for seed in seed_range[0]..(seed_range[0] + seed_range[1]) {
//             seeds.push(seed);
//         }
//     }

//     let mut current_group = "None";
//     let mut seed_to_soil_map: Vec<Vec<u64>> = Vec::new();
//     let mut soil_to_fertilizer_map: Vec<Vec<u64>> = Vec::new();
//     let mut fertilizer_to_water_map: Vec<Vec<u64>> = Vec::new();
//     let mut water_to_light_map: Vec<Vec<u64>> = Vec::new();
//     let mut light_to_temperature_map: Vec<Vec<u64>> = Vec::new();
//     let mut temperature_to_humidity_map: Vec<Vec<u64>> = Vec::new();
//     let mut humidity_to_location_map: Vec<Vec<u64>> = Vec::new();
//     for row in raw_input {
//         if row == "" {
//             continue;
//         }
//         if row == "seed-to-soil map:" {
//             current_group = "seed-to-soil map";
//             continue;
//         }
//         if row == "soil-to-fertilizer map:" {
//             current_group = "soil-to-fertilizer map";
//             continue;
//         }
//         if row == "fertilizer-to-water map:" {
//             current_group = "fertilizer-to-water map";
//             continue;
//         }
//         if row == "water-to-light map:" {
//             current_group = "water-to-light map";
//             continue;
//         }
//         if row == "light-to-temperature map:" {
//             current_group = "light-to-temperature map";
//             continue;
//         }
//         if row == "temperature-to-humidity map:" {
//             current_group = "temperature-to-humidity map";
//             continue;
//         }
//         if row == "humidity-to-location map:" {
//             current_group = "humidity-to-location map";
//             continue;
//         }
//         if current_group == "seed-to-soil map" {
//             seed_to_soil_map.push(
//                 row.split(" ").collect::<Vec<&str>>()
//                 .iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>()
//             );
//         }
//         if current_group == "soil-to-fertilizer map" {
//             soil_to_fertilizer_map.push(
//                 row.split(" ").collect::<Vec<&str>>()
//                 .iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>()
//             );
//         }
//         if current_group == "fertilizer-to-water map" {
//             fertilizer_to_water_map.push(
//                 row.split(" ").collect::<Vec<&str>>()
//                 .iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>()
//             );
//         }
//         if current_group == "water-to-light map" {
//             water_to_light_map.push(
//                 row.split(" ").collect::<Vec<&str>>()
//                 .iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>()
//             );
//         }
//         if current_group == "light-to-temperature map" {
//             light_to_temperature_map.push(
//                 row.split(" ").collect::<Vec<&str>>()
//                 .iter().map(|x| x.parse
//                 ::<u64>().unwrap()).collect::<Vec<u64>>()
//             );
//         }
//         if current_group == "temperature-to-humidity map" {
//             temperature_to_humidity_map.push(
//                 row.split(" ").collect::<Vec<&str>>()
//                 .iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>()
//             );
//         }
//         if current_group == "humidity-to-location map" {
//             humidity_to_location_map.push(
//                 row.split(" ").collect::<Vec<&str>>()
//                 .iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>()
//             );
//         }
//     }

//     for seed in seeds {
//         let mut soil: u64 = 0;
//         let mut fertilizer: u64 = 0;
//         let mut water: u64 = 0;
//         let mut light: u64 = 0;
//         let mut temperature: u64 = 0;
//         let mut humidity: u64 = 0;
//         let mut location: u64 = 0;

//         for row in &seed_to_soil_map {
//             if seed >= row[1] && seed < row[1] + row[2] {
//                 soil = row[0] + (seed - row[1]);
//                 break;
//             }
//         }
//         if soil == 0 {
//             soil = seed;
//         }

//         for row in &soil_to_fertilizer_map {
//             if soil >= row[1] && soil < row[1] + row[2] {
//                 fertilizer = row[0] + (soil - row[1]);
//                 break;
//             }
//         }
//         if fertilizer == 0 {
//             fertilizer = soil;
//         }

//         for row in &fertilizer_to_water_map {
//             if fertilizer >= row[1] && fertilizer < row[1] + row[2] {
//                 water = row[0] + (fertilizer - row[1]);
//                 break;
//             }
//         }
//         if water == 0 {
//             water = fertilizer;
//         }

//         for row in &water_to_light_map {
//             if water >= row[1] && water < row[1] + row[2] {
//                 light = row[0] + (water - row[1]);
//                 break;
//             }
//         }
//         if light == 0 {
//             light = water;
//         }

//         for row in &light_to_temperature_map {
//             if light >= row[1] && light < row[1] + row[2] {
//                 temperature = row[0] + (light - row[1]);
//                 break;
//             }
//         }
//         if temperature == 0 {
//             temperature = light;
//         }

//         for row in &temperature_to_humidity_map {
//             if temperature >= row[1] && temperature < row[1] + row[2] {
//                 humidity = row[0] + (temperature - row[1]);
//                 break;
//             }
//         }
//         if humidity == 0 {
//             humidity = temperature;
//         }

//         for row in &humidity_to_location_map {
//             if humidity >= row[1] && humidity < row[1] + row[2] {
//                 location = row[0] + (humidity - row[1]);
//                 break;
//             }
//         }
//         if location == 0 {
//             location = humidity;
//         }

//         if p2 == 0 {
//             p2 = location;
//         } else {
//             p2 = cmp::min(p2, location);
//         }
//     }
//     p2
// }


fn calc_p2(raw_input: &Vec<String>) -> u64 {
    let mut p2: u64 = 0;

    let seed_ranges = raw_input[0]
        .split(":").collect::<Vec<&str>>()[1].trim()
        .split(" ").collect::<Vec<&str>>()
        .iter().map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>()
    ;
    let mut current_ranges: Ranges = Ranges { ranges: Vec::new() };
    for seed_range in seed_ranges.chunks(2) {
        current_ranges.ranges.push(Range { start_inclusive: seed_range[0], end_exclusive: seed_range[0] + seed_range[1] });
    }
    println!("starting_ranges: {:?}", current_ranges);

    // build a new ranges struct for each map
    let mut mappings: Vec<Mapping> = Vec::new();
    // let mut new_ranges: Vec<Ranges> = Vec::new();
    for row in raw_input.iter().skip(2) {
        if row == "" {
            // perform mapping
            // println!("mappings: {:?}", mappings);
            // mappings.sort_by(|a, b| a.source_start_inclusive.cmp(&b.source_start_inclusive));
            current_ranges.apply_mapping(&mappings);

            // reset mapping
            mappings = Vec::new();
            continue;
        }

        match Mapping::try_from(row) {
            Ok(mapping) => {
                mappings.push(mapping);
            },
            Err(_) => {
                continue
            }
        }
    }
    // perform mapping one final time (due to lack of empty line at end of input)
    print!("ranges: {:?}", current_ranges);
    p2
}


#[derive(Debug)]
#[derive(Clone)]
struct Range {
    start_inclusive: u64,
    end_exclusive: u64
}


#[derive(Debug)]
struct Ranges {
    ranges: Vec<Range>
}


impl Ranges {
    fn apply_mapping(&mut self, mappings: &Vec<Mapping>) {
        let mut new_ranges: Vec<Range> = Vec::new();
        let mut old_ranges: Vec<Range> = Vec::new();
        // println!("applying mapping: {:?} to {:?}", mapping, self.ranges);
        // order mappings by mapping.source_start_inclusive

        for map in mappings {
            while let Some(range) = self.ranges.pop() {
                // does map overlap range?
                if map.source_end_exclusive <= range.start_inclusive || map.source_start_inclusive >= range.end_exclusive {
                    old_ranges.push(range);
                    continue;
                }

                let overlap_start_inclusive = cmp::max(range.start_inclusive, map.source_start_inclusive);
                let overlap_end_exclusive = cmp::min(range.end_exclusive, map.source_end_exclusive);
                // let overlap = Range { start_inclusive: overlap_start_inclusive, end_exclusive: overlap_end_exclusive };

                if overlap_start_inclusive > range.start_inclusive {
                    old_ranges.push(Range { start_inclusive: range.start_inclusive, end_exclusive: overlap_start_inclusive });
                }
                if overlap_end_exclusive < range.end_exclusive {
                    old_ranges.push(Range { start_inclusive: overlap_end_exclusive, end_exclusive: range.end_exclusive });
                }

                let offset = overlap_start_inclusive - map.source_start_inclusive;
                let new_start_inclusive = map.target_start_inclusive + offset;
                let new_end_exclusive = new_start_inclusive + (overlap_end_exclusive - overlap_start_inclusive);
                new_ranges.push(Range { start_inclusive: new_start_inclusive, end_exclusive: new_end_exclusive });
            }
            self.ranges = old_ranges.clone();
        }

        while let Some(range) = old_ranges.pop() {
            new_ranges.push(range);
        }


        // for range in &self.ranges {
        //     println!("range: {:?}", range);
        //     for map in mappings {
        //         println!("map: {:?}", map);
        //         // is map below range?
        //         if map.source_end_exclusive <= range.start_inclusive {
        //             println!("map is below range");
        //             continue;
        //         }
        //         // is map above range?
        //         if map.source_start_inclusive >= range.end_exclusive {
        //             println!("map is above range");
        //             break;
        //         }

        //         println!("map overlaps range");

        //         // find overlap between range and map
        //         let overlap_start_inclusive = cmp::max(range.start_inclusive, map.source_start_inclusive);
        //         let overlap_end_exclusive = cmp::min(range.end_exclusive, map.source_end_exclusive);
        //         let overlap = Range { start_inclusive: overlap_start_inclusive, end_exclusive: overlap_end_exclusive };

        //         if overlap_start_inclusive > range.start_inclusive {
        //             new_ranges.push(Range { start_inclusive: range.start_inclusive, end_exclusive: overlap_start_inclusive });
        //         }
        //         // if overlap_end_exclusive < range.end_exclusive {
        //         //     new_ranges.push(Range { start_inclusive: overlap_end_exclusive, end_exclusive: range.end_exclusive });
        //         // }

        //         let offset = overlap.start_inclusive - map.source_start_inclusive;
        //         let new_start_inclusive = map.target_start_inclusive + offset;
        //         let new_end_exclusive = new_start_inclusive + (overlap.end_exclusive - overlap.start_inclusive);
        //         new_ranges.push(Range { start_inclusive: new_start_inclusive, end_exclusive: new_end_exclusive });
        //     }
        // }
        // self.ranges = new_ranges;
        // println!("new_ranges: {:?}", self.ranges);
    }
}


#[derive(Debug)]
struct Mapping {
    target_start_inclusive: u64,
    target_end_exclusive: u64,
    source_start_inclusive: u64,
    source_end_exclusive: u64
}


impl TryFrom<&String> for Mapping {
    type Error = &'static str;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        if value.split(" ").collect::<Vec<&str>>().len() != 3 {
            return Err("Mapping must have 3 values");
        } else {
            let step = value.split(" ").collect::<Vec<&str>>()[2].parse::<u64>().unwrap();
            let target_start_inclusive = value.split(" ").collect::<Vec<&str>>()[0].parse::<u64>().unwrap();
            let target_end_exclusive = target_start_inclusive + step;
            let source_start_inclusive = value.split(" ").collect::<Vec<&str>>()[1].parse::<u64>().unwrap();
            let source_end_exclusive = source_start_inclusive + step;
            Ok(Mapping { target_start_inclusive, target_end_exclusive, source_start_inclusive, source_end_exclusive})
        }
    }
}


fn get_example_input() -> Vec<String> {
    let test_input: Vec<String> = vec![
        "seeds: 79 14 55 13".to_string(),
        "".to_string(),
        "seed-to-soil map:".to_string(),
        "50 98 2".to_string(),
        "52 50 48".to_string(),
        "".to_string(),
        "soil-to-fertilizer map:".to_string(),
        "0 15 37".to_string(),
        "37 52 2".to_string(),
        "39 0 15".to_string(),
        "".to_string(),
        "fertilizer-to-water map:".to_string(),
        "49 53 8".to_string(),
        "0 11 42".to_string(),
        "42 0 7".to_string(),
        "57 7 4".to_string(),
        "".to_string(),
        "water-to-light map:".to_string(),
        "88 18 7".to_string(),
        "18 25 70".to_string(),
        "".to_string(),
        "light-to-temperature map:".to_string(),
        "45 77 23".to_string(),
        "81 45 19".to_string(),
        "68 64 13".to_string(),
        "".to_string(),
        "temperature-to-humidity map:".to_string(),
        "0 69 1".to_string(),
        "1 0 69".to_string(),
        "".to_string(),
        "humidity-to-location map:".to_string(),
        "60 56 37".to_string(),
        "56 93 4".to_string()
    ];
    test_input
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_p1() {
        let example_input = get_example_input();
        assert_eq!(calc_p1(&example_input), 35);
    }

    #[test]
    fn test_calc_p2() {
        let example_input = get_example_input();
        assert_eq!(calc_p2(&example_input), 46);
    }
}
