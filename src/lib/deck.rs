use rand::Rng;
use crate::card::Card;

struct Deck;


impl Deck {
    pub fn draw_card(self) {
        let random_value: u8 = rand::thread_rng().gen_range(1..14);
        let random_suit = rand::thread_rng().gen_range(1..5);
        let suit_value = ["Spades", "Clubs", "Hearts", "Diamonds"];

        Card { random_value, suit_value[random_suit - 1] };
    }
}