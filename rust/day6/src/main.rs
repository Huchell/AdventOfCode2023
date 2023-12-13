use std::fs;

fn main() {
    let input = get_input();

    let part_1_result = part_1(&input);
    println!("Part 1 result: {part_1_result}");

    let part_2_result = part_2(&input);
    println!("Part 2 result: {part_2_result}");
}

fn get_input() -> String {
    String::from_utf8(fs::read("input").unwrap()).unwrap()
}

fn part_1(input: &str) -> usize {
    let races = parse_input_part_1(input);
    races.iter().map(|race| race.possible_win_count()).product()
}

fn parse_input_part_1(input: &str) -> Vec<Race> {
    let mut lines = input.lines().map(|line| {
        let (_, values) = line.split_once(':').unwrap();
        values
            .trim()
            .split(' ')
            .filter(|value| value.trim().len() > 0)
            .map(|value| value.trim().parse().unwrap())
    });
    let mut times = lines.next().unwrap();
    let mut distance = lines.next().unwrap();

    let mut races = vec![];
    while let Some(time) = times.next() {
        let distance = distance
            .next()
            .expect("time and distance size should always be the same");

        let race = Race { time, distance };
        races.push(race);
    }
    races
}

fn part_2(input: &str) -> usize {
    let race = parse_input_part_2(input);
    race.possible_win_count()
}

fn parse_input_part_2(input: &str) -> Race {
    let mut lines = input.lines().map(|line| {
        let (_, values) = line.split_once(':').unwrap();
        values
            .trim()
            .split(' ')
            .fold(String::new(), |mut acc, value| {
                acc.push_str(value.trim());
                acc
            })
            .parse()
            .unwrap()
    });

    let time = lines.next().unwrap();
    let distance = lines.next().unwrap();
    Race { time, distance }
}

#[derive(Debug, PartialEq, Eq)]
struct Race {
    pub time: usize,
    pub distance: usize,
}

impl Race {
    pub fn possible_win_count(&self) -> usize {
        (1..self.time - 1)
            .filter(|time| {
                let time_left = self.time - time;
                time * time_left > self.distance
            })
            .count()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn part_1_test() {
        let input = "
Time:      7  15   30
Distance:  9  40  200";

        let result = part_1(input.trim());

        let expect = 288;
        assert_eq!(result, expect);
    }

    #[test]
    pub fn part_2_test() {
        let input = "
Time:      7  15   30
Distance:  9  40  200";

        let result = part_2(input.trim());

        let expected = 71503;
        assert_eq!(result, expected);
    }

    #[test]
    pub fn parse_input_test() {
        let input = "
Time:      7  15   30
Distance:  9  40  200";

        let result = parse_input_part_1(input.trim());

        let expected = vec![
            Race {
                time: 7,
                distance: 9,
            },
            Race {
                time: 15,
                distance: 40,
            },
            Race {
                time: 30,
                distance: 200,
            },
        ];
        assert_eq!(result, expected);
    }

    #[test]
    pub fn race_possible_win_count_test() {
        let races = vec![
            Race {
                time: 7,
                distance: 9,
            },
            Race {
                time: 15,
                distance: 40,
            },
            Race {
                time: 30,
                distance: 200,
            },
        ];

        let result = vec![
            races[0].possible_win_count(),
            races[1].possible_win_count(),
            races[2].possible_win_count(),
        ];

        let expected = vec![4, 8, 9];
        assert_eq!(result, expected);
    }
}
