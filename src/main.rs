// use druid::{AppLauncher, WindowDesc};
#[derive(Clone)]
struct Card {
    suit: String,
    value: String,
}
impl Card {
    fn read(&self) -> String {
        return format!("{} of {}", self.value, self.suit);
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
    // fn populate(&self) {
    //     let suits: Vec<String> = ["Spades", "Clubs", "Hearts", "Diamonds"]
    //         .map(String::from)
    //         .to_vec();
    //     let values: Vec<String> = [
    //         "Ace", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "Jack",
    //         "Queen", "King",
    //     ]
    //     .map(String::from)
    //     .to_vec();
    //     let mut cards = Vec::new();
    //     for suit in suits.iter() {
    //         for value in values.iter() {
    //             let card = format!("{} of {}", suit, value);
    //             // println!("{}",card);
    //             cards.push(card)
    //         }
    //     }
    //     self.cards=cards
    // }
}
fn main() {
    // let card= Card{
    //     suit:"spades".to_string(),
    //     value:"ace".to_string()
    // };
    let suits: Vec<String> = ["Spades", "Clubs", "Hearts", "Diamonds"]
        .map(String::from)
        .to_vec();
    let values: Vec<String> = [
        "Ace", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "Jack",
        "Queen", "King",
    ]
    .map(String::from)
    .to_vec();
    let mut cards_test = Vec::new();
    for suit_name in suits.iter() {
        for value_name in values.iter() {
            let card= Card{suit:suit_name.to_string(), value:value_name.to_string()};
            // println!("{}",card);
            cards_test.push(card);
        }
    }
    let deck = Deck{cards:cards_test};
    deck.shuffle()
}
