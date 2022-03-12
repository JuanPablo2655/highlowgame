use crate::lib::card::*;
use std::collections::VecDeque;

pub struct Deck {
    cards: VecDeque<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = VecDeque::new();
        for value in 1..=13 {
            for suit in Suit::all() {
                cards.push_back(Card::new(value, suit))
            }
        }
        Self { cards }
    }

    fn from_cards(cards: &[Card]) -> Self {
        let cards = cards.iter().cloned().collect();
        Self { cards }
    }

    pub fn draw_card(&mut self) -> Option<Card> {
        self.cards.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deck_from_cards() {
        let mut deck =
            Deck::from_cards(&[Card::new(1, Suit::Spades), Card::new(2, Suit::Diamonds)]);
        assert_eq!(deck.draw_card(), Some(Card::new(1, Suit::Spades)));
        assert_eq!(deck.draw_card(), Some(Card::new(2, Suit::Diamonds)));
        assert_eq!(deck.draw_card(), None);
    }
    #[test]
    fn test_deck() {
        let mut deck = Deck::new();
        // the two same cards cannot be drawn
        assert_ne!(deck.draw_card(), deck.draw_card());
    }
}
