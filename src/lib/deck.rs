use crate::lib::card::*;
use rand::seq::SliceRandom;
use rand::Rng;

pub struct Deck;

impl Deck {
    pub fn draw_card(&self) -> Card {
        let random_value: u8 = rand::thread_rng().gen_range(1..14);
        let mut rng = rand::thread_rng();
        let suits = Suit::all();
        let suit = suits.choose(&mut rng).unwrap();

        Card::new(random_value, *suit)
    }
}
