use rand:: {thread_rng, seq::SliceRandom};

#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

impl Deck {
    fn new() -> Self {
        let suites = ["Heats", "Spades", "Diamonds"];
        let values = ["Ace", "Two", "Three"];
        let mut cards = vec![];
        
        for suite in suites {
            for value in values {
                let card = format!("{} of {}", value, suite);
                cards.push(card);
        
            }
        }
        
        let deck = Deck { cards };
        return deck;
    }

    fn shuffle(&mut self){
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng)

    }

    fn deal(&mut self, num_cards: usize) -> Vec<String> {
        self.cards.split_off({
            self.cards.len() - num_cards
        })
    }
}

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
    println!("Heres your deck: {:#?}",deck);
    let cards = deck.deal(2);
    println!("Heres your hand: {:#?}",cards);
}

