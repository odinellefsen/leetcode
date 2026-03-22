export class DealCards {
	public generateDeck(): string[][] {
		const suits = ["hearts", "diamonds", "clubs", "spades"];
		const values = ["7", "8", "9", "10", "J", "Q", "K", "A"];

		do {
			const deck: string[] = [];

			// Generate full deck
			for (const suit of suits) {
				for (const value of values) {
					deck.push(`${value} of ${suit}`);
				}
			}

			// Shuffle the deck
			for (let i = deck.length - 1; i > 0; i--) {
				const j = Math.floor(Math.random() * (i + 1));
				[deck[i], deck[j]] = [deck[j], deck[i]];
			}

			// Check if at least one player would have 5+ cards of a suit
			const hands = [
				deck.slice(0, 8), // Player 1's hand
				deck.slice(8, 16), // Player 2's hand
				deck.slice(16, 24), // Player 3's hand
				deck.slice(24, 32), // Player 4's hand
			];

			// Check if any hand has 5+ cards of any suit
			const isValidDeal = hands.some((hand) => {
				return suits.some((suit) => {
					const suitCount = hand.filter((card) => card.includes(suit)).length;
					return suitCount >= 5;
				});
			});

			if (isValidDeal) {
				return hands;
			}
			// If not valid, loop will continue and generate a new deck
			// biome-ignore lint/correctness/noConstantCondition: <explanation>
		} while (true);
	}
}

const deck = new DealCards().generateDeck();
console.log(deck);
