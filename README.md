# Installing the environment

Should install Rust and NodeJS environment.

# Running the Project

This project was bootstrapped with Cargo - Rust Package Manager

## Available Scripts

In the project directory, you can run:

### `npm start`

Runs the app in the console mode.\
You may also see build steps in the console.

### `npm test`

Launches the test runner in the interactive watch mode.\
There're 9 core test cases for checking `Straight Flush`, `Four of a Kind`, `Full House`, `Flush`, `Straight`, `Three of a Kind`, `Two Pair`, `One Pair` and `High Cards`.

# Implementation

## Project Structure (src/)

### `card.rs`

    Card component with name (A, 2, 3, 4, ... K) and suit (♣, ♦, ♥, ♠)

### `dealer.rs`

    Dealer component which handles shuffling and dealing the cards

### `engine.rs`

    Core engine component for evaluating rank and the best hand.

### `game.rs`

    Game component which manages overall players, a dealer, a deck of cards and a core game engine.

### `player.rs`

    Player component with name, age and a hand (e.g. 5 cards)

### `lib.rs`

    Library module wrapping the whole functionality

### `main.rs`

    Starting point of the project which launches the Poker game

## Design Considerations

    Rust is a statically-typed programming language designed for performance and safety, especially safe concurrency and memory management.

### Object-orientation

### Data Structure

<code>
> Game {
    method: GameType,
    deck: VecDeque<Card>,
    players: Vec<Player>,
    dealer: Dealer,
    engine: Engine
}
</code>

### Separation of Concerns

### Maintainability
