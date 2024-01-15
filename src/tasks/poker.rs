use std::{collections::HashMap, str::FromStr, ptr::null, cmp::Ordering};

#[derive(PartialEq, Clone, Copy, Debug, PartialOrd, Ord, Eq)]
enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(PartialEq, Debug, Clone, Copy)]
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


fn n_of_a_kind(mut hand: Vec<Card>, n: usize) -> bool {
    hand.sort_by_key(|card| card.rank);
    for i in (0..hand.len() - n).step_by(n) {
        let first_card = &hand[i];
        if hand[i..i + n].iter().all(|card| card.rank == first_card.rank) {
            return true;
        }
    }

    false
}

fn five_of_a_kind(hand: Vec<Card>) -> bool {
    n_of_a_kind(hand, 5)
}


fn four_of_a_kind(hand: Vec<Card>) -> bool {
    n_of_a_kind(hand, 4)
}


fn three_of_a_kind(hand: Vec<Card>) -> bool {
    n_of_a_kind(hand, 3)
}


fn full_house(mut hand: Vec<Card>) -> bool {
    hand.sort_by_key(|card| card.rank);

    hand[0].rank == hand[1].rank && hand[1].rank == hand[2].rank && hand[3].rank == hand[4].rank ||
        hand[0].rank == hand[1].rank && hand[2].rank == hand[3].rank && hand[3].rank == hand[4].rank

}


fn flush(hand: Vec<Card>) -> bool {
    hand.iter().all(|card| card.suit == hand[0].suit)
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

fn highest_rank(mut hand: Vec<Card>) -> Card {
    hand.sort_by_key(|card| card.rank);
    *hand.last().unwrap()
}


fn straigh(mut hand: Vec<Card>) -> bool {

    hand.sort_by_key(|card| card.rank);

    dbg!(&hand);

    for i in 1..hand.len() {

        let first_card = &hand[i - 1];
        let second_card = &hand[i];
        // dbg!(first_card.rank, second_card.rank, first_card.rank as u8, second_card.rank as u8);
        if first_card.rank as u8 != (second_card.rank as u8) - 1{
            dbg!(first_card.rank, second_card.rank, first_card.rank as u8, second_card.rank as u8);

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

    let winning_variants = vec![
        five_of_a_kind,
        straigh_of_flush,
        four_of_a_kind,
        full_house,
        flush,
        straigh,
    ];

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
