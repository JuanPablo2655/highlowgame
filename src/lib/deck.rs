use crate::lib::card::*;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for value in 1..=13 {
            for suit in Suit::all() {
                cards.push(Card::new(value, suit))
            }
        }
        Self { cards }
    }

    fn from_cards(cards: &[Card]) -> Self {
        let cards = cards.to_vec();
        Self { cards }
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        let card = self.cards.first().cloned();
        self.cards.remove(0);
        card
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deck_from_cards() {
        let mut deck =
            Deck::from_cards(&[Card::new(1, Suit::Spades), Card::new(2, Suit::Diamonds)]);
        assert_eq!(deck.draw_card().unwrap(), Some(Card::new(1, Suit::Spades)));
        assert_eq!(
            deck.draw_card().unwrap(),
            Some(Card::new(2, Suit::Diamonds))
        );
        assert_eq!(deck.draw_card().unwrap(), None);
    }
    #[test]
    fn test_deck() {
        let mut deck = Deck::new();
        // the two same cards cannot be drawn
        assert_ne!(deck.draw_card().unwrap(), deck.draw_card().unwrap());
    }
}
