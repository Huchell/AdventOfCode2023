use std::fs;

fn main() {
    let input = get_input();

    let part_1_result: usize = valid_game_ids(&input).sum();
    println!("Part 1 result: {}", part_1_result);

    let part_2_result: usize = game_powers(&input).sum();
    println!("Part 2 result: {}", part_2_result);
}

pub fn get_input() -> String {
    String::from_utf8(fs::read("input").unwrap()).unwrap()
}

static RED: usize = 12;
static GREEN: usize = 13;
static BLUE: usize = 14;

pub fn valid_game_ids<'a>(input: &'a str) -> impl Iterator<Item = usize> + 'a {
    input.lines()
        .filter_map(|line| {
            let mut split = line.split(':');
            let id = parse_id(split.next().unwrap());
            if is_rounds_valid(split.next().unwrap()) {
                Some(id)
            } else {
                None
            }
        })
}

#[derive(Default)]
struct Game {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
}

pub fn game_powers<'a>(input: &'a str) -> impl Iterator<Item = usize> + 'a {
    input.lines().map(|line| {
        let game = line.rsplit(':')
            .next()
            .unwrap()
            .split(';')
            .fold(Game::default(), |mut game, input| {
                for round in input.split(',') {
                    let (amount, color) = parse_round(round.trim());
                    match color {
                        "red" => game.red = game.red.max(amount),
                        "green" => game.green = game.green.max(amount),
                        "blue" => game.blue = game.blue.max(amount),
                        _ => {}
                    }
                }
                game
            });
        game.red * game.green * game.blue
    })
}

fn parse_id(input: &str) -> usize {
    input.rsplit(' ').next().unwrap().parse().unwrap()
}

fn is_rounds_valid(input: &str) -> bool {
    input.split(';').all(|round| {
        round.split(',').all(|round| {
            let round = round.trim();
            let mut info = round.split(' ');
            let amount: usize = info.next().unwrap().parse().unwrap();
            match info.next().unwrap() {
                "red" => amount <= RED,
                "green" => amount <= GREEN,
                "blue" => amount <= BLUE,
                _ => false,
            }
        })
    })
}

fn parse_round(input: &str) -> (usize, &str) {
    let mut info = input.split(' ');
    let amount: usize = info.next().unwrap().parse().unwrap();
    (amount, info.next().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::{valid_game_ids, game_powers};

    #[test]
    pub fn valid_game_ids_test() {
       let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

       let result: Vec<usize> = valid_game_ids(input).collect();

       let expected = vec![1, 2, 5];
       assert_eq!(result, expected);
    }

    #[test]
    pub fn game_powers_test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let result: Vec<usize> = game_powers(input).collect();

        let expected = vec![48, 12, 1560, 630, 36];
        assert_eq!(result, expected);
    }
}
