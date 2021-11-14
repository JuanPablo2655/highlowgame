use crate::lib::card::Card;
use rand::Rng;

pub struct Deck;

impl Deck {
    pub fn draw_card(self) -> Card {
        let random_value: u8 = rand::thread_rng().gen_range(1..14);
        let random_suit = rand::thread_rng().gen_range(1..5);
        let suit_value = ["Spades", "Clubs", "Hearts", "Diamonds"];

        Card {
            value: random_value,
            suit: suit_value[random_suit - 1].to_string(),
        }
    }
}
