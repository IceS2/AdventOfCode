use std::{path::Path, fs::File, io::{self, BufRead}, collections::HashMap, cmp::Ordering};

// Entrypoint
fn main() {
    let input = read_input("input.txt").unwrap();
    let mut hand_list: HandList = HandList::new();

    for line in input {
        let split_line: Vec<String> = line.unwrap().split(' ').map(|s| s.to_string()).collect();

        let cards = split_line[0].clone();
        let bet = split_line[1].parse().unwrap();

        hand_list.add_hand(Hand::new(cards, bet))
    }

    println!("{:?}", hand_list.calculate_winnings());
}

fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
// ---------------------------------------------------------------------------------------------

// Card
#[derive(Debug, Clone, Eq)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two
}

impl Card {
    fn strength(&self) -> usize {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Jack => 11,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
        }
    }
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        match c {
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
            _ => panic!()
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
// ---------------------------------------------------------------------------------------------

// HandType
#[derive(Debug, Clone, Eq)]
enum HandType {
    FiveOfAKind(Vec<Card>),
    FourOfAKind(Vec<Card>),
    FullHouse(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    TwoPair(Vec<Card>),
    OnePair(Vec<Card>),
    HighCard(Vec<Card>)
}

impl HandType {
    fn strength(&self) -> usize {
        match self {
            HandType::FiveOfAKind(_) => 7,
            HandType::FourOfAKind(_) => 6,
            HandType::FullHouse(_) => 5,
            HandType::ThreeOfAKind(_) => 4,
            HandType::TwoPair(_) => 3,
            HandType::OnePair(_) => 2,
            HandType::HighCard(_) => 1,
        }
    }

    fn cards(&self) -> &Vec<Card> {
        match self {
            HandType::FiveOfAKind(cards) => cards,
            HandType::FourOfAKind(cards) => cards,
            HandType::FullHouse(cards) => cards,
            HandType::ThreeOfAKind(cards) => cards,
            HandType::TwoPair(cards) => cards,
            HandType::OnePair(cards) => cards,
            HandType::HighCard(cards) => cards,
        }
    }
}

impl From<String> for HandType {
    fn from(s: String) -> Self {
        let hand_map: Vec<(usize, usize)> = s.chars()
            .fold(HashMap::<char, usize>::new(), |mut acc, c| {
                {
                    acc.entry(c).and_modify(|e| *e += 1).or_insert(1);
                }
                acc
            })
            .iter()
            .fold(HashMap::<usize, usize>::new(), |mut acc, (_card, count)| {
                {
                    acc.entry(*count).and_modify(|e| *e += 1).or_insert(1);
                }
                acc
            })
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect();

        let cards: Vec<Card> = s.chars()
            .map(|c| c.into())
            .collect();

        if hand_map.contains(&(5, 1)) {
            Self::FiveOfAKind(cards)
        } else if hand_map.contains(&(4, 1)) {
            Self::FourOfAKind(cards)
        } else if hand_map.contains(&(3, 1)) && hand_map.contains(&(2, 1)) {
            Self::FullHouse(cards)
        } else if hand_map.contains(&(3, 1)) {
            Self::ThreeOfAKind(cards)
        } else if hand_map.contains(&(2, 2)) {
            Self::TwoPair(cards)
        } else if hand_map.contains(&(2, 1)) {
            Self::OnePair(cards)
        } else if hand_map.contains(&(1, 5)) {
            Self::HighCard(cards)
        } else {
            panic!("Your parsing is wrong ;P");
        }
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_strength = self.strength();
        let other_strength = other.strength();
        if self_strength != other_strength {
            self_strength.cmp(&other_strength)
        } else {
            let mut result: Ordering = Ordering::Equal;
            for (card, other_card) in self.cards().iter().zip(other.cards().iter()) {
                let card_strength = card.strength();
                let other_card_strength = other_card.strength();
                if card_strength == other_card_strength {
                    continue;
                } else {
                    result = card.cmp(other_card);
                    break;
                }
            }
            result
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandType {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
// ---------------------------------------------------------------------------------------------

// Hand
#[derive(Debug, Clone, Eq)]
struct Hand {
    cards: HandType,
    bet: u32
}

impl Hand {
    fn new(cards: String, bet: u32) -> Self {
        Self {
            cards: cards.into(),
            bet
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cards.cmp(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}
// ---------------------------------------------------------------------------------------------

// HandList
#[derive(Debug)]
struct HandList {
    hands: Vec<Hand>
}

impl HandList {
    fn new() -> Self {
        Self {
            hands: vec![]
        }
    }

    fn add_hand(&mut self, hand: Hand) {
        self.hands.push(hand);
    }

    fn calculate_winnings(&self) -> u32 {
        let mut hands = self.hands.clone();
        hands.sort();

        hands.iter().enumerate().map(|(index, hand)| hand.bet * (index + 1) as u32).sum()
    }
}
// ---------------------------------------------------------------------------------------------
