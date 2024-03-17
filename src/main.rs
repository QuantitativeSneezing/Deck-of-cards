// use druid::{AppLauncher, WindowDesc};

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
    fn shuffle(){
        println!("shuffling!");
      }
}
fn main() {
    let mut card= Card{
        suit:"spades".to_string(),
        value:"ace".to_string()
    };
    println!("{}",card.read());
    let mut cards_test = Vec::new();
    cards_test.push(card);
    let mut deck= Deck{
        cards:cards_test
    };
}
