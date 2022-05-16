use std::collections::{BTreeSet, HashMap};

use crate::card::{Card, Name, Suit};

pub struct Engine {}

#[derive(PartialEq, PartialOrd, Debug)]
pub enum Rank {
    Unset = -1,
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfKind = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    FourOfKind = 7,
    StraightFlush = 8,
    RoyalFlush = 9,
}

/// get paired vector
pub fn get_paired_vec(cards: &Vec<Name>) -> Vec<u8> {
    let mut tmp: Vec<Name>;
    tmp = cards.clone();
    tmp.sort();

    let mut res: Vec<u8> = Vec::new();

    for i in 0..tmp.len() {
        if i > 0 && tmp[i] == tmp[i - 1] {
            res.push(tmp[i] as u8 + 50);
        } else {
            res.push(tmp[i] as u8);
        }
    }

    res.sort_by(|a, b| b.cmp(a));

    return res;
}

/// compare arrays
pub fn compare_arrays(arr1: &Vec<u8>, arr2: &Vec<u8>) -> i32 {
    for (a, b) in arr1.iter().zip(arr2.iter()) {
        if a > b {
            return 1;
        } else if a < b {
            return -1;
        }
    }
    return 0;
}

impl Engine {
    /// constructor
    pub fn new() -> Engine {
        Engine {}
    }

    /// evaluate hand of 5 cards
    /// return score (StraightFlush, Four of a Kind, ... High Card)
    pub fn evaluate(&self, hand: &Vec<Card>) -> Rank {
        if let Some(rank) = self.check_flush(&hand) {
            rank
        } else if self.is_straight(&hand) {
            Rank::Straight
        } else {
            self.other_rank(&hand)
        }
    }

    /// finds the most frequent suit
    pub fn find_suit(&self, hand: &Vec<Card>) -> Option<Suit> {
        let mut suit_count: HashMap<Suit, u8> = HashMap::new();

        hand.iter().map(|card| card.suit).max_by_key(|&suit| {
            let count: &mut u8 = suit_count.entry(suit).or_insert(0);
            *count += 1;
            *count
        })
    }

    /// check flush
    pub fn check_flush(&self, hand: &Vec<Card>) -> Option<Rank> {
        let some_suit: Option<Suit> = self.find_suit(&hand);

        let suit = some_suit.unwrap();
        let flush: Vec<Card> = hand
            .iter()
            .filter(|&card| card.suit == suit)
            .map(|&card| card)
            .collect();

        if flush.len() < 5 {
            return None;
        }

        if !self.is_straight(&hand) {
            return Some(Rank::Flush);
        }

        return Some(Rank::StraightFlush);
    }

    /// check straight
    pub fn is_straight(&self, hand: &Vec<Card>) -> bool {
        if hand.len() < 5 {
            return false;
        }

        let mut names: Vec<Name> = hand.iter().map(|card| card.name).collect();
        names.sort();
        names.dedup();

        return names.len() == 5 && names[4] as u8 - names[0] as u8 == 4;
    }

    /// calc other rank types
    pub fn other_rank(&self, hand: &Vec<Card>) -> Rank {
        let mut value_count: HashMap<Name, u8> = HashMap::new();
        let mut counts: HashMap<u8, u8> = (1..=4).map(|i| (i, 0)).collect();

        hand.iter()
            .for_each(|card| *value_count.entry(card.name).or_insert(0) += 1);
        value_count
            .values()
            .for_each(|&i| *counts.entry(i).or_insert(0) += 1);

        if counts[&4] > 0 {
            return Rank::FourOfKind;
        }

        match (counts[&3], counts[&2]) {
            (0, 0) => Rank::HighCard,
            (0, 1) => Rank::OnePair,
            (0, _) => Rank::TwoPair,
            (1, 0) => Rank::ThreeOfKind,
            (_, _) => Rank::FullHouse,
        }
    }

    pub fn compare_hands(&self, hand1: &Vec<Card>, hand2: &Vec<Card>) -> i32 {
        let rank1 = self.evaluate(hand1);
        let rank2 = self.evaluate(hand2);

        if rank1 > rank2 {
            return 1;
        } else if rank1 < rank2 {
            return -1;
        } else {
            if rank1 == Rank::StraightFlush
                || rank1 == Rank::FourOfKind
                || rank1 == Rank::FullHouse
                || rank1 == Rank::Straight
                || rank1 == Rank::ThreeOfKind
            {
                let max1 = hand1.iter().map(|card| card.name).max();
                let max2 = hand2.iter().map(|card| card.name).max();
                if max1 > max2 {
                    return 1;
                } else if max1 < max2 {
                    return -1;
                } else {
                    return 0;
                }
            } else if rank1 == Rank::Flush || rank1 == Rank::HighCard {
                let mut sort1: Vec<Name> = hand1.iter().map(|card| card.name).collect();
                let mut sort2: Vec<Name> = hand2.iter().map(|card| card.name).collect();

                sort1.sort_by(|a, b| b.cmp(a));
                sort2.sort_by(|a, b| b.cmp(a));

                for (c1, c2) in sort1.iter().zip(sort2.iter()) {
                    if c1 > c2 {
                        return 1;
                    } else if c1 < c2 {
                        return -1;
                    }
                }
                return 0;
            } else if rank1 == Rank::TwoPair || rank1 == Rank::OnePair {
                let mut sort1: Vec<Name> = hand1.iter().map(|card| card.name).collect();
                let mut sort2: Vec<Name> = hand2.iter().map(|card| card.name).collect();

                let mut pair1 = get_paired_vec(&sort1);
                let mut pair2 = get_paired_vec(&sort2);

                return compare_arrays(&pair1, &pair2);
            } else {
                return 0;
            }
        }
    }
}
