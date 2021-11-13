pub struct Card {
    value: u8,
    suit: str,
}

impl Card {
    pub fn new(u8 value, str suit) -> self {
        self {
            value,
            suit,
        }
    }

    pub fn get_value(self) -> u8 {
        self.value;
    }

    pub fn get_suit(self) -> str {
        self.suit;
    }

    pub fn delcare_card(self) -> str {
        if self.value == 11 {
            format!("the Jack of {}", self.suit)
        } else if self.value == 12 {
            format!("the Queen of {}", self.suit)
        } else if self.value == 13 {
            format!("the King of {}", self.suit)
        } else if self.value == 1 {
            format!("the Ace of {}", self.suit)
        } else {
            format!("the {} of {}", self.value, self.suit)
        }
    }
}