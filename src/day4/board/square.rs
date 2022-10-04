pub struct Square {
    value: u8,
    marked: bool,
}

impl Square {
    pub fn new() -> Square {
        return Square {
            value: 0,
            marked: false,
        }
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    pub fn get_value(&self) -> u8 {
        return self.value;
    }
}
