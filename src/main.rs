// use druid::{AppLauncher, WindowDesc};

struct Card {
    suit: String,
    value: String
}
struct Deck {
  cards:Vec<Card>
}
impl Deck{
    fn shuffle(){
        println!("shuffling!")
      }
}
fn main() {
    let mut card= Card{
        suit:"spades".to_string(),
        value:"ace".to_string()
    };
    let mut deck=
    // let mut deck= Deck();
}
