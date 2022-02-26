pub struct Move {
    target_square: u8,
    current_square: u8,
    piece_id: u8,
    move_str: Option<String>,
}

impl Move {
    pub fn new(target: u8, current: u8, id: u8) -> Move {
        Move {
            target_square: target,
            current_square: current,
            piece_id: id,
            move_str: None,
        }
    }

    pub fn set_str(&mut self, move_s: &str) {
        self.move_str = Some(move_s.to_string());
    }

    pub fn get_str(&self) -> &str {
        self.move_str
            .as_ref()
            .map(|m| m.as_str())
            .unwrap_or("")
    }

    fn tup(&self) -> (u8, u8, u8) {
        (self.current_square, self.target_square, self.piece_id)
    }
}
