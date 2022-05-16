extern crate poker;

#[cfg(test)]
mod tests {
    use poker::card::{Card, Name::*, Suit::*};
    use poker::engine::{Engine, Rank::*};

    #[test]
    fn straight_flush() {
        let mut hand: Vec<Card> = Vec::new();
        let engine: Engine = Engine::new();

        hand.append(&mut vec![
            Card {
                name: AceHigh,
                suit: Diamond,
            },
            Card {
                name: King,
                suit: Diamond,
            },
            Card {
                name: Queen,
                suit: Diamond,
            },
            Card {
                name: Jack,
                suit: Diamond,
            },
            Card {
                name: Ten,
                suit: Diamond,
            },
        ]);
        assert_eq!(engine.evaluate(&hand), StraightFlush);
    }

    #[test]
    fn four_of_kind() {
        let mut hand: Vec<Card> = Vec::new();
        let engine: Engine = Engine::new();

        hand.append(&mut vec![
            Card {
                name: King,
                suit: Diamond,
            },
            Card {
                name: King,
                suit: Spade,
            },
            Card {
                name: King,
                suit: Club,
            },
            Card {
                name: King,
                suit: Heart,
            },
            Card {
                name: Ten,
                suit: Diamond,
            },
        ]);
        assert_eq!(engine.evaluate(&hand), FourOfKind);
    }

    #[test]
    fn full_house() {
        let mut hand: Vec<Card> = Vec::new();
        let engine: Engine = Engine::new();

        hand.append(&mut vec![
            Card {
                name: King,
                suit: Diamond,
            },
            Card {
                name: King,
                suit: Spade,
            },
            Card {
                name: King,
                suit: Club,
            },
            Card {
                name: Ten,
                suit: Heart,
            },
            Card {
                name: Ten,
                suit: Diamond,
            },
        ]);
        assert_eq!(engine.evaluate(&hand), FullHouse);
    }

    #[test]
    fn flush() {
        let mut hand: Vec<Card> = Vec::new();
        let engine: Engine = Engine::new();

        hand.append(&mut vec![
            Card {
                name: King,
                suit: Diamond,
            },
            Card {
                name: Queen,
                suit: Diamond,
            },
            Card {
                name: Two,
                suit: Diamond,
            },
            Card {
                name: Ten,
                suit: Diamond,
            },
            Card {
                name: Nine,
                suit: Diamond,
            },
        ]);
        assert_eq!(engine.evaluate(&hand), Flush);
    }

    #[test]
    fn straight() {
        let mut hand: Vec<Card> = Vec::new();
        let engine: Engine = Engine::new();

        hand.append(&mut vec![
            Card {
                name: King,
                suit: Heart,
            },
            Card {
                name: Queen,
                suit: Heart,
            },
            Card {
                name: Jack,
                suit: Spade,
            },
            Card {
                name: Ten,
                suit: Club,
            },
            Card {
                name: Nine,
                suit: Diamond,
            },
        ]);
        assert_eq!(engine.evaluate(&hand), Straight);
    }

    #[test]
    fn three_of_kind() {
        let mut hand: Vec<Card> = Vec::new();
        let engine: Engine = Engine::new();

        hand.append(&mut vec![
            Card {
                name: King,
                suit: Diamond,
            },
            Card {
                name: Queen,
                suit: Diamond,
            },
            Card {
                name: Two,
                suit: Diamond,
            },
            Card {
                name: King,
                suit: Spade,
            },
            Card {
                name: King,
                suit: Club,
            },
        ]);
        assert_eq!(engine.evaluate(&hand), ThreeOfKind);
    }

    #[test]
    fn two_pair() {
        let mut hand: Vec<Card> = Vec::new();
        let engine: Engine = Engine::new();

        hand.append(&mut vec![
            Card {
                name: King,
                suit: Diamond,
            },
            Card {
                name: King,
                suit: Club,
            },
            Card {
                name: Two,
                suit: Diamond,
            },
            Card {
                name: Two,
                suit: Spade,
            },
            Card {
                name: Nine,
                suit: Diamond,
            },
        ]);
        assert_eq!(engine.evaluate(&hand), TwoPair);
    }

    #[test]
    fn one_pair() {
        let mut hand: Vec<Card> = Vec::new();
        let engine: Engine = Engine::new();

        hand.append(&mut vec![
            Card {
                name: King,
                suit: Diamond,
            },
            Card {
                name: King,
                suit: Club,
            },
            Card {
                name: Three,
                suit: Diamond,
            },
            Card {
                name: Two,
                suit: Spade,
            },
            Card {
                name: Nine,
                suit: Diamond,
            },
        ]);
        assert_eq!(engine.evaluate(&hand), OnePair);
    }

    #[test]
    fn high_card() {
        let mut hand: Vec<Card> = Vec::new();
        let engine: Engine = Engine::new();

        hand.append(&mut vec![
            Card {
                name: King,
                suit: Diamond,
            },
            Card {
                name: Seven,
                suit: Club,
            },
            Card {
                name: Four,
                suit: Diamond,
            },
            Card {
                name: Two,
                suit: Spade,
            },
            Card {
                name: Nine,
                suit: Diamond,
            },
        ]);
        assert_eq!(engine.evaluate(&hand), HighCard);
    }
}
