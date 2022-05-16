use crate::{card::Card, engine::Rank};

pub struct Player {
    pub hand: Vec<Card>,
    pub rank: Rank,
    pub name: String,
    pub age: u8,
}

impl Player {
    /// construtor
    pub fn new(name: &String, age: u8) -> Player {
        Player {
            hand: Vec::new(),
            name: name.clone(),
            age: age,
            rank: Rank::Unset,
        }
    }

    /// print status
    pub fn print(&self) {
        print!("{}: ", self.name);
        for card in self.hand.iter() {
            print!("{} ", card.custom_fmt());
        }
        match self.rank {
            Rank::Unset => print!("Unset"),
            Rank::HighCard => print!("High Card"),
            Rank::OnePair => print!("One Pair"),
            Rank::TwoPair => print!("Two Pair"),
            Rank::ThreeOfKind => print!("Three of Kind"),
            Rank::Straight => print!("Straight"),
            Rank::Flush => print!("Flush"),
            Rank::FullHouse => print!("Full House"),
            Rank::FourOfKind => print!("Four of Kind"),
            Rank::StraightFlush => print!("Straight Flush"),
            Rank::RoyalFlush => print!("Royal Flush"),
        }
    }

    /// clear cards
    pub fn clear(&mut self) {
        self.hand.clear();
    }

    /// add card
    pub fn add_card(&mut self, card: &Card) {
        self.hand.push(*card);
    }
}
