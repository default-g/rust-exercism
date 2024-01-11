use std::{collections::HashMap, str::FromStr, ptr::null};

#[derive(PartialEq, Clone, Copy, Debug, PartialOrd)]
enum Rank {
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

#[derive(PartialEq, Clone, Copy, Debug)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(PartialEq, Debug)]
struct Card {
    rank: Rank,
    suit: Suit,
}

#[derive(Debug, PartialEq, Eq)]
struct CardParseError;

impl FromStr for Card {

    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let rank_map : HashMap<&str, Rank> = HashMap::from([
            ("2", Rank::Two),
            ("3", Rank::Three),
            ("4", Rank::Four),
            ("5", Rank::Five),
            ("6", Rank::Six),
            ("7", Rank::Seven),
            ("8", Rank::Eight),
            ("9", Rank::Nine),
            ("10", Rank::Ten),
            ("J", Rank::Jack),
            ("Q", Rank::Queen),
            ("K", Rank::King),
            ("A", Rank::Ace),
        ]);

        let suit_map : HashMap<&str, Suit> = HashMap::from([
            ("C", Suit::Clubs),
            ("D", Suit::Diamonds),
            ("H", Suit::Hearts),
            ("S", Suit::Spades),
        ]);

        let rank: Rank = rank_map.get(&s[0..1]).unwrap_or(&Rank::Ten).clone();

        let suit_slice: &str;
        if rank == Rank::Ten {
            suit_slice = &s[2..3];
            dbg!(suit_slice);
        } else {
            suit_slice = &s[1..2];
        }

        Ok(Card {
           rank,
            suit: match suit_map.get(suit_slice) {
                Some(suit) => *suit,
                None => Suit::Hearts,
            },
        })

    }
}

fn five_of_a_kind(hand: Vec<Card>) -> bool {
    let first_card = &hand[0];
    hand.iter().all(|card| card.rank == first_card.rank)
}

fn four_of_a_kind(hand: Vec<Card>) -> bool {
    unimplemented!();
}

fn full_house(hand: Vec<Card>) -> bool {
    unimplemented!();
}

fn flush(hand: Vec<Card>) -> bool {
    unimplemented!();
}

fn three_of_a_kind(hand: Vec<Card>) -> bool {
    unimplemented!();
}

fn two_pair(hand: Vec<Card>) -> bool {
    unimplemented!();
}

fn one_pair(hand: Vec<Card>) -> bool {
    unimplemented!();
}

fn high_card(hand: Vec<Card>) -> bool {
    unimplemented!();
}

fn straigh(hand: Vec<Card>) -> bool {

    for i in 1..hand.len() {

        let first_card = &hand[i - 1];
        let second_card = &hand[i];

        if first_card.rank > second_card.rank || first_card.rank == Rank::Ace && second_card.rank == Rank::Two && i == 1 {
            return false;
        }
    }

    true
}


fn straigh_of_flush(hand: Vec<Card>) -> bool {
    let first_card = &hand[0];
    if !hand.iter().all(|card| card.suit == first_card.suit) {
        return false;
    }

    straigh(hand)
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut winning_hand_vector: Vec<&'a str> = Vec::new();

    let winning_variants = vec![five_of_a_kind, straigh_of_flush, straigh];

    for variant in winning_variants {
        for hand in hands {
            let cards: Vec<Card> = hand.split(' ')
                .map(|card| Card::from_str(card).unwrap())
                .collect();

            if variant(cards) {
                winning_hand_vector.push(hand);
            }
        }
        if !winning_hand_vector.is_empty() {
            break;
        }
    }

    winning_hand_vector
}
