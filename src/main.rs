struct Deck {
    cards: Vec<Card>,
}
struct Card {
    value: u8,
    suit: String,
}

fn main() {
    let mut deck = Deck { cards: Vec::new() };
    deck.cards.push(Card {
        value: 1,
        suit: "Spades".to_string(),
    });
    deck.cards.push(Card {
        value: 2,
        suit: "Hearts".to_string(),
    });
    deck.cards.push(Card {
        value: 3,
        suit: "Clubs".to_string(),
    });
    deck.cards.push(Card {
        value: 4,
        suit: "Diamonds".to_string(),
    });
    deck.cards.push(Card {
        value: 5,
        suit: "Spades".to_string(),
    });
    println!("Hello, world!");
}
