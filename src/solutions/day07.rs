use std::cmp::Ordering;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(s: char) -> Card {
        match s {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("invalid card: {}", s),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, PartialOrd, Ord)]
struct HandCards([Card; 5]);

impl HandCards {
    fn joker_order(&self, other: &Self) -> Ordering {
        for (card, other_card) in self.0.iter().zip(other.0.iter()) {
            let order = match (card, other_card) {
                (Card::Jack, Card::Jack) => Ordering::Equal,
                (Card::Jack, _) => Ordering::Less,
                (_, Card::Jack) => Ordering::Greater,
                _ => card.cmp(other_card),
            };

            if order != Ordering::Equal {
                return order;
            }
        }

        Ordering::Equal
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Hand {
    cards: HandCards,
    interpretation: HandInterpretation,
    bet: usize,
    with_jokers: bool,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum HandInterpretation {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_histogram(mut cards: [Card; 5], with_jokers: bool) -> ([usize; 5], usize) {
    cards.sort();

    let mut histogram = [0; 5];
    let mut joker_count = 0;

    let mut last = cards[0];
    let mut variant_count = 0;

    for card in cards {
        if with_jokers && card == Card::Jack {
            joker_count += 1;
            continue;
        }

        if card != last {
            variant_count += 1;
            last = card;
        }

        histogram[variant_count] += 1;
    }

    histogram.sort();
    histogram.reverse();

    (histogram, joker_count)
}

impl HandInterpretation {
    fn from_cards(cards: [Card; 5], with_jokers: bool) -> HandInterpretation {
        let (counts, joker_count) = get_histogram(cards, with_jokers);

        let has_card_with_count = |count: usize| -> bool { counts[0] + joker_count >= count };

        if has_card_with_count(5) {
            return HandInterpretation::FiveOfAKind;
        }

        if has_card_with_count(4) {
            return HandInterpretation::FourOfAKind;
        }

        let has_full_house = {
            let required_jokers = 3 - counts[0].min(3) + 2 - counts[1].min(2);
            required_jokers <= joker_count
        };

        if has_full_house {
            return HandInterpretation::FullHouse;
        }

        if has_card_with_count(3) {
            return HandInterpretation::ThreeOfAKind;
        }

        let has_two_pair = {
            let required_jokers = 2 - counts[0].min(2) + 2 - counts[1].min(2);
            required_jokers <= joker_count
        };

        if has_two_pair {
            return HandInterpretation::TwoPair;
        }

        if has_card_with_count(2) {
            return HandInterpretation::OnePair;
        }

        HandInterpretation::HighCard
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.interpretation != other.interpretation {
            return (self.interpretation as usize).cmp(&(other.interpretation as usize));
        }

        if self.with_jokers {
            self.cards.joker_order(&other.cards)
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}

impl Hand {
    fn new(cards: [Card; 5], bet: usize, with_jokers: bool) -> Hand {
        Self {
            cards: HandCards(cards),
            interpretation: HandInterpretation::from_cards(cards, with_jokers),
            bet,
            with_jokers,
        }
    }
}

fn parse_input(inpuit: &str) -> impl Iterator<Item = ([Card; 5], usize)> + '_ {
    inpuit.lines().map(|line| {
        let (cards, bet) = line.split_once(' ').expect("invalid format");

        assert_eq!(cards.len(), 5, "invalid number of cards");

        let mut parsed_cards = [Card::Two; 5];

        for (i, card) in cards.chars().enumerate() {
            parsed_cards[i] = Card::from_char(card);
        }

        let bet = bet.parse().expect("invalid bet");

        (parsed_cards, bet)
    })
}

fn solve(input: &str, with_jokers: bool) -> u32 {
    let mut input: Vec<_> = parse_input(input)
        .map(|(cards, bet)| Hand::new(cards, bet, with_jokers))
        .collect();

    input.sort();

    input
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bet)
        .sum::<usize>() as u32
}

pub fn part_one(input: &str) -> u32 {
    solve(input, false)
}

pub fn part_two(input: &str) -> u32 {
    solve(input, true)
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("examples", 7);
        assert_eq!(part_one(&input), 6440);

        let input = read_file("inputs", 7);
        assert_eq!(part_one(&input), 252656917);
    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 7);
        assert_eq!(part_two(&input), 5905);

        let input = read_file("inputs", 7);
        assert_eq!(part_two(&input), 253499763);
    }
}
