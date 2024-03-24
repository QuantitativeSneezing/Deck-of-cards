// use druid::{AppLauncher, WindowDesc};
#[derive(Clone)]
struct Card {
    suit: i8,
    value: i8,
}
impl Card {
    fn read(&self) -> String {
        let suits: Vec<String> = ["Spades", "Clubs", "Hearts", "Diamonds"]
            .map(String::from)
            .to_vec();
        let values: Vec<String> = [
            "Ace", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "Jack",
            "Queen", "King",
        ]
        .map(String::from)
        .to_vec();
        return format!(
            "{} of {}",
            values[self.value as usize], suits[self.suit as usize]
        );
    }
}
struct Deck {
    cards: Vec<Card>,
}
impl Deck {
    fn shuffle(&self) {
        println!("shuffling!");
        let card = &self.cards[0];
        println!("{} is on top", card.read());
    }
    // fn repopulate(&self) {

    //     let mut new_cards = Vec::new();
    //     for suit_name in suits.iter() {
    //         for value_name in values.iter() {
    //             let card= Card{suit:suit_name.to_string(), value:value_name.to_string()};
    //             // println!("{}",card);
    //             new_cards.push(card);
    //         }
    //     }

    //     *self.cards= new_cards;
    // }
}
fn main() {
    // let card= Card{
    //     suit:"spades".to_string(),
    //     value:"ace".to_string()
    // };
    // let suits: Vec<String> = ["Spades", "Clubs", "Hearts", "Diamonds"]
    //     .map(String::from)
    //     .to_vec();
    // let values: Vec<String> = [
    //     "Ace", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "Jack",
    //     "Queen", "King",
    // ]
    // .map(String::from)
    // .to_vec();
    let mut cards_test = Vec::new();
    for suit_int in 0..3 {
        for value_int in 0..12 {
            let card = Card {
                suit: suit_int,
                value: value_int,
            };
            // println!("{}",card);
            cards_test.push(card);
        }
    }
    let deck = Deck { cards: cards_test };
    deck.shuffle()
}
