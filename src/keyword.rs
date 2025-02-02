use rand::Rng as _;

/// Can I buy a real D256?
pub struct Die {
    value: u8,
}

impl Die {
    pub fn roll() -> Self {
        let value = rand::thread_rng().gen();
        Self { value }
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}
