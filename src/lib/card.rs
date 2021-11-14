pub struct Card {
    value: u8,
    suit: String,
}

impl Card {
    pub fn new(value: u8, suit: String) -> Self {
        Self { value, suit }
    }

    pub fn get_value(&self) -> &u8 {
        &self.value
    }

    pub fn get_suit(&self) -> &str {
        &self.suit
    }

    pub fn declare_card(&self) -> &str {
        let value = match &self.value {
            11 => "Jack",
            12 => "Queen",
            13 => "King",
            1 => "Ace",
            _ => &self.value.to_string(),
        };
        format!("the {} of {}", value, &self.suit)
    }
}
