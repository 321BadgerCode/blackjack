# Blackjack

This is a simple command-line Blackjack game implemented in Rust. The player plays against the dealer with ASCII-art cards.

## Features
- Fully playable Blackjack game.
- ASCII-art representation of playing cards.
- Dealer follows standard Blackjack rules (hits below 17, stands otherwise).
- Basic user input for hit or stand.

## Requirements
- Rust (latest stable version recommended)

## Installation
1. Clone this repository:
	```sh
	git clone https://github.com/321BadgerCode/blackjack.git
	cd ./blackjack/
	```
2. Build the project:
	```sh
	cargo build --release
	```
3. Run the game:
	```sh
	cargo run
	```

## How to Play
- The player starts with two cards, and the dealer has one visible card.
- The player can choose to "hit" (draw another card) or "stand" (keep their current hand).
- The goal is to get as close to 21 as possible without exceeding it.
- Face cards (J, Q, K) are worth 10 points, and an Ace (A) can be worth 1 or 11.
- If the player exceeds 21, they bust and lose the game.
- After the player stands, the dealer plays according to the rules and the winner is determined.

## License
This project is open-source and available under the MIT License.

## Contributions
Contributions are welcome! Feel free to fork the repository and submit a pull request.