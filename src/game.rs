extern crate rand;

use std::collections::VecDeque;

use crate::{
    card::{Card, Name, Name::*, Suit, Suit::*},
    dealer::Dealer,
    engine::{Engine, Rank},
    player::Player,
};

#[derive(Ord, PartialEq, PartialOrd, Eq, Hash, Clone, Copy)]
pub enum GameType {
    FiveDraw,
}

pub struct Game {
    method: GameType,
    deck: VecDeque<Card>,
    players: Vec<Player>,
    dealer: Dealer,
    engine: Engine,
}

impl Game {
    /// constructor
    pub fn new() -> Game {
        Game {
            deck: build_deck(),
            players: Vec::new(),
            method: GameType::FiveDraw,
            dealer: Dealer::new(),
            engine: Engine::new(),
        }
    }

    /// get winner
    pub fn get_winner(&mut self) -> u8 {
        // evaluate cards
        let mut found = false;
        let mut winner: &mut Player = &mut Player::new(&String::from(""), 0);
        let mut index = 0;
        let mut winner_id = 0;
        for player in &mut self.players {
            player.rank = self.engine.evaluate(&player.hand);
            if !found || self.engine.compare_hands(&winner.hand, &player.hand) < 0 {
                winner = player;
                found = true;
                winner_id = index;
            }
            index = index + 1;
        }
        winner_id
    }

    /// initialize players
    pub fn init_players(&mut self) {
        self.players.clear();
        if self.method == GameType::FiveDraw {
            self.players.push(Player {
                hand: Vec::new(),
                rank: Rank::Unset,
                name: String::from("Player 1"),
                age: 30,
            });
            self.players.push(Player {
                hand: Vec::new(),
                rank: Rank::Unset,
                name: String::from("Player 2"),
                age: 40,
            });
            self.players.push(Player {
                hand: Vec::new(),
                rank: Rank::Unset,
                name: String::from("Player 3"),
                age: 20,
            });
            self.players.push(Player {
                hand: Vec::new(),
                rank: Rank::Unset,
                name: String::from("Player 4"),
                age: 25,
            });
            self.players.push(Player {
                hand: Vec::new(),
                rank: Rank::Unset,
                name: String::from("Player 5"),
                age: 35,
            });
        }
    }

    /// play FiveDraw game
    pub fn play_five_draw(&mut self) {
        self.init_players();
        loop {
            // shuffle the cards
            self.deck = self.dealer.shuffle(self.deck.clone());

            // deal
            self.dealer.deal(&self.deck, &mut self.players);

            // print result
            self.print_result();

            break;
        }
    }

    /// print result
    pub fn print_result(&mut self) {
        let winner_id = self.get_winner();

        let mut index: u8 = 0;
        for player in &self.players {
            player.print();

            if winner_id == index {
                print!(" Winner!");
            } else if self
                .engine
                .compare_hands(&self.players[winner_id as usize].hand, &player.hand)
                == 0
            {
                print!(" Winner!");
            }
            println!();

            index = index + 1;
        }
    }
}

/// build deck of 52 cards
fn build_deck() -> VecDeque<Card> {
    let names: [Name; 13] = [
        Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, AceHigh,
    ];
    let suits: [Suit; 4] = [Heart, Diamond, Spade, Club];
    let mut deck: VecDeque<Card> = VecDeque::new();
    suits.iter().for_each(|&suit| {
        names.iter().for_each(|&name| {
            deck.push_back(Card {
                name: name,
                suit: suit,
            });
        });
    });
    deck
}
