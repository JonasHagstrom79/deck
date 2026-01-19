#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suits = ["Spades", "Hearts", "Diamonds", "Clubs"];
        let ranks = ["Ace", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"];
        let mut cards = vec![];

        for suit in suits {
            for rank in ranks {
                let card = format!("{} of {}", rank, suit);
                cards.push(card);
            }
        }
        Deck { cards }
    }

    fn shuffel(self) {

    }
}

fn main() {
    let deck = Deck::new();
    deck.shuffel();
    println!("Here is your deck: {:#?}", deck);
}
