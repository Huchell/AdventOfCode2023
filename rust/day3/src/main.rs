use std::fs;

fn main() {
    let input = get_input();

    let schematic = parse_schematic(&input);

    let part_1: usize = schematic
        .iter()
        .flat_map(|(_, parts)| parts)
        .sum();
    println!("Part 1 result: {}", part_1);

    let part_2: usize = schematic
        .iter()
        .filter_map(|(i, parts)| match &input[*i..i+1] {
            "*" => if parts.len() == 2 {
                Some(parts[0] * parts[1])
            } else {
                None
            },
            _=> None,
        })
        .sum();
    println!("Part 2 result: {}", part_2);
}


pub fn get_input() -> String {
    String::from_utf8(fs::read("input").unwrap()).unwrap()
}

pub fn parse_schematic(input: &str) -> Vec<(usize, Vec<usize>)> {
    let width = input.find('\n').expect("end of line") + 1;
    let check_max = input.len();

    input
        .char_indices()
        .filter_map(move |(i, ch)| {
            if !is_symbol(&ch) || ch == '.' {
                return None;
            }

            let top_left = i.saturating_sub(width + 1);
            let top_right = i.saturating_sub(width) + 2;
            let bottom_left = i + width - 1;
            let bottom_right = i + width + 2;

            let check_ranges = [
                (top_left..top_right),
                (i.saturating_sub(1)..check_max.min(i + 2)),
                (check_max.min(bottom_left)..check_max.min(bottom_right)),
            ];
            let iter = check_ranges
                .iter()
                .flat_map(|range| {
                    input[range.start..range.end]
                        .char_indices()
                        .scan(false, |state, (i, ch)| {
                            Some(
                                if *state {
                                    *state = ch.is_digit(10);
                                    None
                                } else {
                                    *state = ch.is_digit(10);
                                    if *state {
                                        Some(i)
                                    } else {
                                        None
                                    }
                                }
                            )
                        })
                        .filter_map(|i| match i {
                            Some(i) => Some(parse_number(input, range.start + i)),
                            None => None,
                        })
                });
            Some((i, iter.collect()))
        })
        .collect()
}

fn parse_number(input: &str, index: usize) -> usize {
    let (start_i, _) = input[..index + 1]
        .char_indices()
        .rev()
        .take_while(|(_, ch)| ch.is_digit(10))
        .last()
        .unwrap();

    input[start_i..input.len()]
        .chars()
        .map_while(|ch| ch.to_digit(10))
        .fold(0, |acc, digit| acc * 10 + digit as usize)
}

static NON_SYMBOLS_CHARS: &[char; 13] = &['.', '\r', '\n', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
fn is_symbol(ch: &char) -> bool {
    !NON_SYMBOLS_CHARS.contains(ch)
}

#[cfg(test)]
mod tests {
    use crate::parse_schematic;

    #[test]
    pub fn parse_schematic_test() {
        let input = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let result = parse_schematic(input.trim());
        
        let expected = vec![
            (14, vec![467, 35]),
            (39, vec![633]),
            (47, vec![617]),
            (60, vec![592]),
            (91, vec![664]),
            (93, vec![755, 598]),
        ];
        assert_eq!(result, expected);
    }
}
