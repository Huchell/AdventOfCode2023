use std::{cmp::Ordering, fmt::Debug, collections::HashMap};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum HandType {
    HighCard = 1,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq, Ord)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub hand_type: HandType,
}

impl Hand {
    pub fn with_cards(cards: Vec<Card>) -> Self {
        let hand_type = Self::get_hand_type(&cards);
        Self {
            cards,
            hand_type,
        }
    }

    pub fn from_str(input: &str) -> Self {
        let cards = input.chars().map(|ch| match ch {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Ten,
            '9' => Card::Nine,
            '8' => Card::Eight,
            '7' => Card::Seven,
            '6' => Card::Six,
            '5' => Card::Five,
            '4' => Card::Four,
            '3' => Card::Three,
            '2' => Card::Two,
            _ => panic!("Cannot map {ch}")
        }).collect();
        let hand_type = Self::get_hand_type(&cards);
        Self { cards, hand_type }
    }

    pub fn from_str_with_jokers(input: &str) -> Self {
        let cards = input
            .chars()
            .map(|ch| match ch {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => Card::Joker,
                'T' => Card::Ten,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                _ => panic!("Cannot map {ch}")
            })
            .collect();
        let hand_type = Self::get_hand_type(&cards);
        Self { cards, hand_type }
    }

    fn get_hand_type(cards: &Vec<Card>) -> HandType {
        let card_counts = cards
            .iter()
            .fold(HashMap::new(), |mut map: HashMap<Card, usize>, card| {
                map.entry(card.clone()).and_modify(|entry| *entry += 1).or_insert(1);
                map
            });

        let result = card_counts
            .iter()
            .filter(|(card, _)| **card != Card::Joker)
            .map(|(_, count)| {
                match count {
                    1 => HandType::HighCard,
                    2 => HandType::OnePair,
                    3 => HandType::ThreeOfAKind,
                    4 => HandType::FourOfAKind,
                    5 => HandType::FiveOfAKind,
                    _ => HandType::HighCard,
                }
            })
            .fold(HandType::HighCard, |cur, hand_type| {
                match (cur, hand_type) {
                    (HandType::OnePair, HandType::OnePair) => HandType::TwoPair,
                    (HandType::OnePair, HandType::ThreeOfAKind) 
                        | (HandType::ThreeOfAKind, HandType::OnePair) => HandType::FullHouse,
                    (cur, hand_type) => if cur >= hand_type {
                        cur
                    } else {
                        hand_type
                    }
                }
            });

        let joker_count = card_counts.get(&Card::Joker).unwrap_or(&0);
        match joker_count {
            1 => match result {
                HandType::HighCard => HandType::OnePair,
                HandType::OnePair => HandType::ThreeOfAKind,
                HandType::TwoPair => HandType::FullHouse,
                HandType::ThreeOfAKind => HandType::FourOfAKind,
                HandType::FourOfAKind => HandType::FiveOfAKind,
                _ => result,
            },
            2 => match result {
                HandType::HighCard => HandType::ThreeOfAKind,
                HandType::OnePair => HandType::FourOfAKind,
                HandType::ThreeOfAKind => HandType::FiveOfAKind,
                _ => result,
            },
            3 => match result {
                HandType::HighCard => HandType::FourOfAKind,
                HandType::OnePair => HandType::FiveOfAKind,
                _ => result,
            },
            4 | 5 => match result {
                HandType::HighCard => HandType::FiveOfAKind,
                _ => result,
            }
            _ => result,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => return Some(Ordering::Less),
            Ordering::Greater => return Some(Ordering::Greater),
            Ordering::Equal => {
                for i in 0..self.cards.len() {
                    let card = self.cards.get(i).unwrap();
                    let other_card = other.cards.get(i).unwrap();

                    match card.cmp(other_card) {
                        Ordering::Equal => continue,
                        Ordering::Less => return Some(Ordering::Less),
                        Ordering::Greater => return Some(Ordering::Greater),
                    };
                }

                Some(Ordering::Equal)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Hand, Card, HandType};

    #[test]
    pub fn hand_from_str() {
        let inputs = vec![
            "32T3K",
            "T55J5",
            "KK677",
            "KTJJT",
            "QQQJA",
        ];

        let results: Vec<_> = inputs
            .iter()
            .map(|input| Hand::from_str(input))
            .collect();

        let expected = vec![
            Hand::with_cards(vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]),
            Hand::with_cards(vec![Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five]),
            Hand::with_cards(vec![Card::King, Card::King, Card::Six, Card::Seven, Card::Seven]),
            Hand::with_cards(vec![Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten]),
            Hand::with_cards(vec![Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace]),
        ];
        assert_eq!(results, expected);
    }

    #[test]
    pub fn hands_sorts_correctly() {
        let mut cards = vec![
            Hand::with_cards(vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]),
            Hand::with_cards(vec![Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five]),
            Hand::with_cards(vec![Card::King, Card::King, Card::Six, Card::Seven, Card::Seven]),
            Hand::with_cards(vec![Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten]),
            Hand::with_cards(vec![Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace]),
        ];
        cards.sort();

        let expected = vec![
            Hand::with_cards(vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King]),
            Hand::with_cards(vec![Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten]),
            Hand::with_cards(vec![Card::King, Card::King, Card::Six, Card::Seven, Card::Seven]),
            Hand::with_cards(vec![Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five]),
            Hand::with_cards(vec![Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace]),
        ];
        assert_eq!(cards, expected);
    }

    #[test]
    pub fn hands() {
        let hand = Hand::with_cards(vec![Card::Eight, Card::Joker, Card::Joker, Card::Joker, Card::Joker]);

        let expected = HandType::FiveOfAKind;
        assert_eq!(hand.hand_type, expected);
    }
}
