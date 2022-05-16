#[derive(Ord, PartialEq, PartialOrd, Eq, Hash, Clone, Copy)]
pub enum Name {
    AceLow = 1,
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
    AceHigh = 14,
}

#[derive(Ord, PartialEq, PartialOrd, Eq, Hash, Clone, Copy)]
pub enum Suit {
    Heart = 1,
    Diamond = 2,
    Spade = 3,
    Club = 4,
}

#[derive(Clone, Copy)]
pub struct Card {
    pub name: Name,
    pub suit: Suit,
}

impl Card {
    /// constructor
    pub fn new(name: Name, suit: Suit) -> Card {
        Card {
            name: name,
            suit: suit,
        }
    }

    /// get formatted string
    pub fn custom_fmt(&self) -> String {
        let name = match self.name {
            Name::AceLow | Name::AceHigh => "A",
            Name::Two => "2",
            Name::Three => "3",
            Name::Four => "4",
            Name::Five => "5",
            Name::Six => "6",
            Name::Seven => "7",
            Name::Eight => "8",
            Name::Nine => "9",
            Name::Ten => "10",
            Name::Jack => "J",
            Name::Queen => "Q",
            Name::King => "K",
        };
        let suit = match self.suit {
            Suit::Heart => "♥",
            Suit::Diamond => "♦",
            Suit::Spade => "♠",
            Suit::Club => "♣",
        };

        format!("{}{}", name.to_string(), suit.to_string())
    }
}
