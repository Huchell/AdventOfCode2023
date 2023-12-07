use std::fs;

fn main() {
    let input = get_input();

    let winning_numbers = parse_winning_numbers(&input);
    let part_1 = get_total_points(winning_numbers.iter());
    println!("Part 1 result: {}", part_1);

    let part_2 = get_total_scratchcards(winning_numbers.iter());
    println!("Part 2 result: {}", part_2);
}

pub fn get_input() -> String {
    String::from_utf8(fs::read("input").unwrap()).unwrap()
}

pub fn parse_winning_numbers(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            let (_, results) = line.split_once(':').unwrap();
            let (winning_numbers, chosen_numbers) = results.split_once('|').unwrap();
            
            let mut winning_numbers: Vec<usize> = winning_numbers
                .split(' ')
                .filter_map(|s| s.parse::<usize>().ok())
                .collect();
            winning_numbers.sort();

            let mut chosen_numbers: Vec<usize> = chosen_numbers
                .split(' ')
                .filter_map(|s| s.trim().parse::<usize>().ok())
                .filter(|num| winning_numbers.contains(num))
                .collect();
            chosen_numbers.sort();
    
            chosen_numbers
        })
        .collect()
}

pub fn get_total_points<'a, Iter>(iter: Iter) -> usize 
where Iter: Iterator<Item = &'a Vec<usize>>
{
    iter
        .filter(|nums| nums.len() > 0)
        .fold(0, |acc, nums| acc + (1 << (nums.len() - 1)))
}

pub fn get_total_scratchcards<'a, Iter>(iter: Iter) -> usize
where Iter: Iterator<Item = &'a Vec<usize>>
{
    iter
        .enumerate()
        .fold(Vec::new(), |mut state, (i, nums)| {
            let amount = match state.get(i) {
                Some(x) => {
                    state[i] = x + 1;
                    state[i]
                },
                None => {
                    state.insert(i, 1);
                    1
                }
            };

            let len = nums.len();
            if len != 0 {
                for i in i+1..i+len+1 {
                    match state.get(i) {
                        Some(x) => state[i] = x + amount,
                        None => state.insert(i, amount),
                    }
                }
            }
            state
        })
        .iter()
        .fold(0, |acc, amount| acc + amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn parse_winning_numbers_test() {
        let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = parse_winning_numbers(input.trim());

        let expected = vec![
            vec![17, 48, 83, 86],
            vec![32, 61],
            vec![1, 21],
            vec![84],
            vec![],
            vec![],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    pub fn get_total_points_test() {
        let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let numbers = parse_winning_numbers(input.trim());
        let result = get_total_points(numbers.iter());

        let expected = 13;
        assert_eq!(result, expected);
    }

    #[test]
    pub fn get_total_scratchcards_test() {
        
        let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let numbers = parse_winning_numbers(input.trim());
        let result = get_total_scratchcards(numbers.iter());

        let expected = 30;
        assert_eq!(result, expected);
    }
}
