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

    // pub fn get_suit(&self) -> &str {
    //     &self.suit
    // }

    pub fn declare_card(&self) -> String {
        let value = match &self.value {
            11 => "Jack".to_string(),
            12 => "Queen".to_string(),
            13 => "King".to_string(),
            1 => "Ace".to_string(),
            _ => self.value.to_string(),
        };
        format!("the {} of {}", value, &self.suit)
    }
}
