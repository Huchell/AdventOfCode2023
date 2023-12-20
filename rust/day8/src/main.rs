use std::{collections::HashMap, fs};

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
    let (sequence, map) = parse_input(input);
    steps(&sequence, &map, "AAA", "ZZZ")
}

fn part_2(input: &str) -> usize {
    let (sequence, map) = parse_input(input);
    map
        .keys()
        .filter(|node| node.ends_with("A"))
        .map(|node| steps(&sequence, &map, node, "Z"))
        .reduce(lcm)
        .unwrap()
}

fn steps(sequence: &Vec<Direction>, map: &HashMap<&str, (&str, &str)>, start: &str, target: &str) -> usize {
    sequence
        .iter()
        .cycle()
        .scan(start, |state, direction| {
            let (left, right) = map.get(state).expect("should always have node");
            let next = match direction {
                Direction::Left => left,
                Direction::Right => right,
            };

            if next.ends_with(target) {
                None
            } else {
                *state = next;
                Some(next)
            }
        })
        .count() + 1
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    match ((a, b), (a & 1, b & 1)) {
        _ if a == b => a,
        ((_, 0), _) => a,
        ((0, _), _) => b,
        (_, (0, 1) | (1, 0)) => gcd(a >> 1, b),
        (_, (0, 0)) => gcd(a >> 1, b >> 1) << 1,
        (_, (1, 1)) => {
            let (a, b) = (a.min(b), a.max(b));
            gcd((b - a) >> 1, a)
        }
        _ => unreachable!(),
    }
}

#[derive(Debug)]
pub enum Direction {
    Left,
    Right,
}

type NodeMap<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse_input(input: &str) -> (Vec<Direction>, NodeMap) {
    let mut lines = input.lines();
    
    let sequence = parse_sequence(lines.next().unwrap());
    lines.next();

    let map = lines.fold(HashMap::new(), |mut map, line| {
        let (node, left, right) = parse_map_line(line);
        map.insert(node, (left, right));
        map
    });

    (sequence, map)
}

fn parse_sequence<'a>(line: &'a str) -> Vec<Direction> {
    line
        .chars()
        .map(|ch| match ch {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>()
}

fn parse_map_line(line: &str) -> (&str, &str, &str) {
    let (node, end) = line.split_once('=').unwrap();
    let (left, right) = end.split_once(',').unwrap();
    (node.trim(), &left.trim()[1..], &right.trim()[..right.len()-2])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn part_1_test() {
        let input = "
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let result = part_1(input.trim());

        let expected = 2;
        assert_eq!(result, expected);
    }

    #[test]
    pub fn part_1_test_looping_sequence() {
        let input = "
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let result = part_1(input.trim());

        let expected = 6;
        assert_eq!(result, expected);
    }

    #[test]
    pub fn part_2_test() {
        let input = "
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";


        let result = part_2(input.trim());

        let expected = 6;
        assert_eq!(result, expected);
    }
}
