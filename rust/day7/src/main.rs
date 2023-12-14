use std::fs;

use hand::Hand;

mod hand;

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

pub fn part_1(input: &str) -> usize {
    let (hands, bets) = parse_input_part_1(input);
    let mut hands: Vec<_> = hands.iter().enumerate().collect();
    hands.sort_by_key(|(_, hand)| *hand);

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (bet_index, _))| acc + (bets.get(*bet_index).unwrap() * (i + 1)))
}

fn parse_input_part_1(input: &str) -> (Vec<Hand>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            let (hand, bet) = line.split_once(' ').unwrap();

            let hand = Hand::from_str(hand);
            (hand, bet.parse().unwrap())
        })
        .fold((vec![], vec![]), |(mut hands, mut bets), (hand, bet)| {
            hands.push(hand);
            bets.push(bet);
            (hands, bets)
        })
}

pub fn part_2(input: &str) -> usize {
    let (hands, bets) = parse_input_part_2(input);
    let mut hands: Vec<_> = hands
        .iter()
        .enumerate()
        .collect();
    hands.sort_by_key(|(_, hand)| *hand);

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (bet_index, _))| acc + (bets.get(*bet_index).unwrap() * (i + 1)))
}

fn parse_input_part_2(input: &str) -> (Vec<Hand>, Vec<usize>) {
    input
        .lines()
        .map(|line| {
            let (hand, bet) = line.split_once(' ').unwrap();

            let hand = Hand::from_str_with_jokers(hand);
            (hand, bet.parse().unwrap())
        })
    .fold((vec![], vec![]), |(mut hands, mut bets), (hand, bet)| {
        hands.push(hand);
        bets.push(bet);
        (hands, bets)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn part_1_test() {
        let input = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = part_1(input.trim());

        let expected = 6440;
        assert_eq!(result, expected);
    }

    #[test]
    pub fn part_2_test() {
        let input = "
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = part_2(input.trim());

        let expected = 5905;
        assert_eq!(result, expected);
    }
}
