use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

#[derive(Debug, Clone, Copy)]
struct Card {
	rank: &'static str,
	suit: &'static str,
	value: u8,
}

const SUITS: [&str; 4] = ["♠", "♥", "♦", "♣"];
const RANKS: [(&str, u8); 13] = [
	("A", 11), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), 
	("8", 8), ("9", 9), ("10", 10), ("J", 10), ("Q", 10), ("K", 10)
];

fn generate_deck() -> Vec<Card> {
	let mut deck = Vec::new();
	for &suit in &SUITS {
		for &(rank, value) in &RANKS {
			deck.push(Card { rank, suit, value });
		}
	}
	deck
}

fn draw_card(deck: &mut Vec<Card>) -> Card {
	deck.pop().unwrap()
}

fn hand_value(hand: &[Card]) -> u8 {
	let mut value = 0;
	let mut aces = 0;
	for card in hand {
		value += card.value;
		if card.rank == "A" {
			aces += 1;
		}
	}
	while value > 21 && aces > 0 {
		value -= 10;
		aces -= 1;
	}
	value
}

fn display_card(card: &Card) -> String {
	let rank_left = if card.rank.len() == 1 { format!("{} ", card.rank) } else { card.rank.to_string() };
	let rank_right = if card.rank.len() == 1 { format!(" {}", card.rank) } else { card.rank.to_string() };
	format!("+------+\n|{}    |\n|  {}   |\n|    {}|\n+------+\n", rank_left, card.suit, rank_right)
}

fn display_hand(hand: &[Card], owner: &str) {
	println!("{}'s hand:", owner);
	for card in hand {
		println!("{}", display_card(card));
	}
	println!("(Value: {})\n", hand_value(hand));
}

fn main() {
	let mut deck = generate_deck();
	deck.shuffle(&mut thread_rng());

	let mut player_hand = vec![draw_card(&mut deck), draw_card(&mut deck)];
	let mut dealer_hand = vec![draw_card(&mut deck), draw_card(&mut deck)];

	loop {
		display_hand(&player_hand, "Player");
		println!("Dealer's hand:");
		println!("{}\n+------+
|  ?   |
|  ?   |
|  ?   |
+------+\n", display_card(&dealer_hand[0]));

		if hand_value(&player_hand) > 21 {
			println!("Bust! You lose.");
			return;
		}

		println!("Hit or stand? (h/s)");
		let mut choice = String::new();
		io::stdin().read_line(&mut choice).unwrap();
		let choice = choice.trim();

		if choice == "h" {
			player_hand.push(draw_card(&mut deck));
		} else {
			break;
		}
	}

	while hand_value(&dealer_hand) < 17 {
		dealer_hand.push(draw_card(&mut deck));
	}

	display_hand(&dealer_hand, "Dealer");
	let player_total = hand_value(&player_hand);
	let dealer_total = hand_value(&dealer_hand);

	if dealer_total > 21 || player_total > dealer_total {
		println!("You win!");
	} else if player_total < dealer_total {
		println!("Dealer wins.");
	} else {
		println!("It's a tie.");
	}
}