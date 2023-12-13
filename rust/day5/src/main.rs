use std::{ops::Range, fs};

fn main() {
    let input = get_input();

    let almanac = Almanac::from_str(&input);

    let part_1 = part_1(&almanac);
    println!("Part 1 result: {part_1}");

    let part_2 = part_2(&almanac);
    println!("Part 2 result: {part_2}");
}

fn get_input() -> String {
    String::from_utf8(fs::read("input").unwrap()).unwrap()
}

fn part_1(almanac: &Almanac) -> usize {
    almanac.seeds
        .iter()
        .map(|seed| almanac.get_location(seed))
        .min()
        .unwrap()
}

fn part_2(almanac: &Almanac) -> usize {
    let mut seeds = almanac.seeds.iter();
    let mut total_seeds = vec![];
    while let Some(&seed) = seeds.next() {
        let range = seeds.next().unwrap();
        total_seeds.push(seed..seed+range);
    }

    (0..).find_map(|x| {
        let humidity = almanac.humidity_to_location.get_from(&x);
        let temperature = almanac.temperature_to_humidity.get_from(&humidity);
        let light = almanac.light_to_temperature.get_from(&temperature);
        let water = almanac.water_to_light.get_from(&light);
        let fertilizer = almanac.fertilizer_to_water.get_from(&water);
        let soil = almanac.soil_to_fertilizer.get_from(&fertilizer);
        let seed = almanac.seed_to_soil.get_from(&soil);

        if total_seeds.iter().any(|s| s.contains(&seed)) {
            Some(x)
        } else {
            None
        }
    }).unwrap()
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
        let fertilizer = self.soil_to_fertilizer.get_to(&soil);
        let water = self.fertilizer_to_water.get_to(&fertilizer);
        let light = self.water_to_light.get_to(&water);
        let temperature = self.light_to_temperature.get_to(&light);
        let humidity = self.temperature_to_humidity.get_to(&temperature);
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
pub struct Map {
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

    pub fn get_to(&self, from_id: &usize) -> usize {
        match self.get_range_id(&self.from, from_id) {
            Some(range_id) => {
                let from_range = self.from.get(range_id).unwrap();
                let to_range = self.to.get(range_id).unwrap();

                let i = from_id - from_range.start;
                to_range.start + i
            },
            None => from_id.clone(),
        }
    }

    pub fn get_from(&self, to_id: &usize) -> usize {
        match self.get_range_id(&self.to, to_id) {
            Some(range_id) => {
                let from_range = self.from.get(range_id).unwrap();
                let to_range = self.to.get(range_id).unwrap();

                let i = to_id - to_range.start;
                from_range.start + i
            },
            None => to_id.clone(),
        }
    }

    pub fn push_from(&mut self, from: Range<usize>) {
        self.from.push(from)
    }

    pub fn push_to(&mut self, to: Range<usize>) {
        self.to.push(to)
    }

    fn get_range_id(&self, ranges: &Vec<Range<usize>>, id: &usize) -> Option<usize> {
        ranges
            .iter()
            .enumerate()
            .find_map(|(i, range)| {
                if range.contains(id) {
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
