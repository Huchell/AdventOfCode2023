use std::ops::Range;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Eq, PartialEq)]
struct Almanac {
    pub seeds: Vec<usize>,
    pub seed_to_soil: Map,
    pub soil_to_fertilizer: Map,
    pub fertilizer_to_water: Map,
    pub water_to_light: Map,
    pub light_to_temperature: Map,
    pub temperature_to_humidity: Map,
    pub humidity_to_location: Map,
}

impl Almanac {
    pub fn from_str(input: &str) -> Self {
        let mut lines = input.lines();

        let seeds = parse_seed_line(lines.next().unwrap());

        let maps = lines.fold(Vec::new(), |mut state, line| {
            if line.len() == 0 {
                return state;
            }

            if line.contains(':') {
                state.push(Map::new());
                return state;
            }

            let mut nums = line.split(' ').map(|s| s.parse::<usize>().unwrap());
            let dest = nums.next().unwrap();
            let source = nums.next().unwrap();
            let range = nums.next().unwrap();

            let i = state.len() - 1;
            let last_map = state.get_mut(i).unwrap();
            last_map.push_from(source..source+range);
            last_map.push_to(dest..dest+range);
            state
        });

        Almanac { 
            seeds, 
            seed_to_soil: maps[0].clone(), 
            soil_to_fertilizer: maps[1].clone(), 
            fertilizer_to_water: maps[2].clone(), 
            water_to_light: maps[3].clone(), 
            light_to_temperature: maps[4].clone(), 
            temperature_to_humidity: maps[5].clone(), 
            humidity_to_location: maps[6].clone(),
        }
    }

    pub fn get_location(&self, seed: &usize) -> usize {
        let soil = self.seed_to_soil.get_to(seed);
        println!();
        let fertilizer = self.fertilizer_to_water.get_to(&soil);
        println!();
        let water = self.fertilizer_to_water.get_to(&fertilizer);
        let light = self.water_to_light.get_to(&water);
        let temperature = self.light_to_temperature.get_to(&light);
        let humidity = self.temperature_to_humidity.get_to(&temperature);
        println!("{}, {}, {}, {}, {}, {}, {}, {}", seed, soil, fertilizer, water, light, temperature, humidity, self.humidity_to_location.get_to(&humidity));
        self.humidity_to_location.get_to(&humidity)
    }
}

fn parse_seed_line(line: &str) -> Vec<usize> {
    let (_, seeds) = line.split_once(':').unwrap();
    seeds.trim().split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect()
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Map {
    from: Vec<Range<usize>>,
    to: Vec<Range<usize>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            from: Vec::new(),
            to: Vec::new(),
        }
    }

    pub fn to_and_from(to: Vec<Range<usize>>, from: Vec<Range<usize>>)  -> Self {
        Self {
            from,
            to,
        }
    }

    pub fn get_to(&self, from_id: &usize) -> usize {
        match self.get_range_id(from_id) {
            Some(range_id) => {
                let from_range = self.from.get(range_id).unwrap();
                let to_range = self.to.get(range_id).unwrap();

                let (i, _) = from_range.clone().enumerate().find(|(_, from_i)| from_i == from_id).unwrap();
                println!("{}-{}, {}-{}, {}", from_range.start, from_range.end, to_range.start, to_range.end, to_range.clone().skip(i).next().unwrap());
                to_range.clone().skip(i).next().unwrap()
            },
            None => from_id.clone(),
        }
    }

    pub fn push_from(&mut self, from: Range<usize>) {
        self.from.push(from)
    }

    pub fn push_to(&mut self, to: Range<usize>) {
        self.to.push(to)
    }

    fn get_range_id(&self, from_id: &usize) -> Option<usize> {
        self.from
            .iter()
            .enumerate()
            .find_map(|(i, range)| {
                if range.contains(from_id) {
                    Some(i)
                } else {
                    None
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    static DEFAULT_INPUT: &str = "
seeds: 79 14 55 13

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
56 93 4";

    #[test]
    pub fn get_location_test() {
        let almanac = Almanac::from_str(DEFAULT_INPUT.trim());

        let result: Vec<_> = almanac.seeds.iter().map(|seed| almanac.get_location(seed)).collect();
        let expected = vec![82, 43, 86, 35];
        assert_eq!(result, expected);
    }
}
