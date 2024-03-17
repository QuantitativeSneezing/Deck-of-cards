// use druid::{AppLauncher, WindowDesc};
#[derive(Clone)]
struct Card {
    suit: String,
    value: String
}
impl Card{
    fn read(&self)->String{
        return format!("{} of {}", self.value, self.suit)
    }
}
struct Deck {
  cards:Vec<Card>
}
impl Deck{
    fn shuffle(&self){
        println!("shuffling!");
        let card= &self.cards[0];
        println!("{} is on top",card.read());
      }
}
fn main() {
    let card= Card{
        suit:"spades".to_string(),
        value:"ace".to_string()
    };
    println!("{}",card.read());
    let mut cards_test = Vec::new();
    cards_test.push(card);
    let deck= Deck{
        cards:cards_test
    };
    deck.shuffle()
}
