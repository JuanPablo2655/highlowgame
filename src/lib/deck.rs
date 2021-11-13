use rand::Rng;
use crate::card::Card;

struct Deck;

impl Deck {
    let mut rng = Rng::thread_rng();
    let random_value = rng.gen_range(1..14);
    let random_suit = rng.gen_range(1, 5);
    let suit_value = ["Spades", "Clubs", "Hearts", "Diamonds"];
    pub fn draw_card(self) {
        card.new(random_value, suit_value[random_suit - 1]);
    }
}