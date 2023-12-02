use std::fs;

fn main() {
    let input = get_input();
    let simple_digits = get_pairs_simple(&input);
    let simple_sum = simple_digits.iter().sum::<u32>();

    println!("Sum of values: {}", simple_sum);

    let complex_digits = get_pairs_complex(&input);
    let complex_sum = complex_digits.iter().sum::<u32>();
    println!("Sum of values 2: {}", complex_sum);
}

pub fn get_input() -> String {
    String::from_utf8(fs::read("input").unwrap()).unwrap()
}

pub fn get_pairs_simple(input: &str) -> Vec<u32> {
    input.lines().map(|line| {
        let first = get_first_digit(line, parse_slice_simple).unwrap();
        let second = get_last_digit(line, parse_slice_simple).unwrap();

        format!("{}{}", first, second).parse().unwrap()
    }).collect()
}

pub fn get_pairs_complex(input: &str) -> Vec<u32> {
    input.lines().map(|line| {
        let first = get_first_digit(line, parse_slice_complex).unwrap();
        let last = get_last_digit(line, parse_slice_complex).unwrap();

        let result = format!("{}{}", first, last);
        result.parse().unwrap()
    }).collect()
}

fn get_first_digit(line: &str, parse_fn: impl Fn(&str) -> Option<char>) -> Option<char> {
    let mut left = 0;
    let mut right = 1;

    let max_right = line.len();
    while left < line.len() {
        let slice = &line[left..right];
        let ch = parse_fn(slice);
        if ch.is_some() {
            return ch;
        }

        right = max_right.min(right + 1);
        if right - left > 5 || right == max_right {
            left += 1;
            right = max_right.min(left + 1);
        }
    }

    None
}

fn get_last_digit(line: &str, parse_fn: impl Fn(&str) -> Option<char>) -> Option<char> {
    let mut left = line.len() - 1;
    let mut right = line.len();

    let min_left = 0;
    while right > 0 {
        let slice = &line[left..right];
        let ch = parse_fn(slice);
        if ch.is_some() {
            return ch;
        }

        left = min_left.max(left.saturating_sub(1));
        if right - left > 5 || left == min_left {
            right -= 1;
            left = min_left.max(right - 1);
        }
    }

    None
}

fn parse_slice_simple(slice: &str) -> Option<char> {
    match slice.parse::<u32>() {
        Ok(digit) => char::from_digit(digit, 10),
        _ => None,
    }
}

static DIGITS: &[char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9'];
static DIGIT_STRINGS: &[&str] = &["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
fn parse_slice_complex(slice: &str) -> Option<char> {
    if slice.len() == 1 {
        match slice.parse::<u32>() {
            Ok(digit) => char::from_digit(digit, 10),
            Err(_) => None
        }
    } else {
        let position = DIGIT_STRINGS.iter().position(|s| s == &slice)?;
        Some(DIGITS[position])
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_pairs_complex, get_pairs_simple};

    #[test]
    pub fn get_pairs_simple_test() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result = get_pairs_simple(input);

        let expected = vec![12, 38, 15, 77];
        assert_eq!(result, expected);
    }

    #[test]
    pub fn get_pairs_complex_test() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let result = get_pairs_complex(input);

        let expected = vec![29, 83, 13, 24, 42, 14, 76];
        assert_eq!(result, expected);
    }
}
