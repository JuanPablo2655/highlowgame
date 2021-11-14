pub struct Card {
    value: u8,
    suit: String,
}

impl Card {
    pub fn new(value: u8 , suit: String) -> Self {
        Self {
            value,
            suit,
        }
    }

    pub fn get_value(self) {
        self.value;
    }

    pub fn get_suit(self) {
        self.suit;
    }

    pub fn delcare_card(self) -> String {
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