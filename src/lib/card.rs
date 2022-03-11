#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Suit {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

impl Suit {
    pub fn all() -> Vec<Self> {
        vec![Self::Spades, Self::Clubs, Self::Hearts, Self::Diamonds]
    }
}

impl ToString for Suit {
    fn to_string(&self) -> String {
        match self {
            Self::Clubs => "Clubs",
            Self::Spades => "Spades",
            Self::Hearts => "Hearts",
            Self::Diamonds => "Diamonds",
        }
        .to_string()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Card {
    value: u8,
    suit: Suit,
}

impl Card {
    pub fn new(value: u8, suit: Suit) -> Self {
        Self { value, suit }
    }

    pub fn get_value(&self) -> &u8 {
        &self.value
    }

    // pub fn get_suit(&self) -> &str {
    //     &self.suit
    // }

    pub fn declare_card(&self) -> String {
        let named_value = match &self.value {
            11 => Some("Jack"),
            12 => Some("Queen"),
            13 => Some("King"),
            1 => Some("Ace"),
            _ => None,
        };
        let value = named_value.unwrap_or(&self.value.to_string()).to_string();
        format!("the {} of {}", value, self.suit.to_string())
    }
}
