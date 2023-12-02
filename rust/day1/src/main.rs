use std::fs;

fn main() {
    let input = get_input();
    let simple_digits = get_pairs(&input, parse_slice_simple);
    let simple_sum: usize = simple_digits.sum();

    println!("Sum of values: {}", simple_sum);

    let complex_digits = get_pairs(&input, parse_slice_complex);
    let complex_sum: usize = complex_digits.sum();
    println!("Sum of values 2: {}", complex_sum);
}

pub fn get_input() -> String {
    String::from_utf8(fs::read("input").unwrap()).unwrap()
}

pub fn get_pairs<'a>(input: &'a str, parse_fn: fn(&str) -> Option<usize>) -> impl Iterator<Item = usize> + 'a {
    input
        .lines()
        .map(move |line| {
            let mut digits = (0..line.len()).map(|i| &line[i..]).flat_map(parse_fn);

            let first = digits
                .next()
                .expect("Always at least 1 digit in input line");
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
}

fn parse_slice_simple(slice: &str) -> Option<usize> {
    slice[0..1].parse().ok()
}

static DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
fn parse_slice_complex(slice: &str) -> Option<usize> {
    slice[0..1].parse().ok().or_else(|| {
        DIGITS
            .iter()
            .enumerate()
            .find(|(_, &digit)| slice.starts_with(digit))
            .map(|(i, _)| i + 1)
    })
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    pub fn get_pairs_simple_test() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let result: Vec<_> = get_pairs(input, parse_slice_simple).collect();

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

        let result: Vec<_> = get_pairs(input, parse_slice_complex).collect();

        let expected = vec![29, 83, 13, 24, 42, 14, 76];
        assert_eq!(result, expected);
    }
}
