use rand::{seq::SliceRandom, thread_rng};
use std::collections::VecDeque;

use crate::{card::Card, player::Player};

pub enum Method {
    Riffle,
}

pub struct Dealer {
    method: Method,
}

impl Dealer {
    /// constructor
    pub fn new() -> Dealer {
        Dealer {
            method: Method::Riffle,
        }
    }

    /// shuffle the cards
    pub fn shuffle(&mut self, deck: VecDeque<Card>) -> VecDeque<Card> {
        let mut deck_vec: Vec<Card> = deck.iter().map(|&c| c).collect();
        deck_vec.shuffle(&mut thread_rng());
        return VecDeque::from(deck_vec);
    }

    /// deal the cards
    pub fn deal(&mut self, deck: &VecDeque<Card>, players: &mut Vec<Player>) {
        let mut count = 0;
        let mut cursor = deck.len() - 1;

        // clear cards for each player
        for player in &mut players.into_iter() {
            player.clear();
        }

        loop {
            if count >= 5 {
                break;
            }
            for player in &mut players.into_iter() {
                player.add_card(&deck[cursor]);
                cursor -= 1;
            }
            count += 1;
        }
    }
}
