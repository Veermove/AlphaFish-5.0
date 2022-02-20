pub use std::collections::HashMap;
pub use super::piece::{Piece};

pub struct Board {
    rep: HashMap<u8, Piece>,
    _prev_rep: HashMap<u8, Piece>,
    fen_rep: Option<String>,
}

impl Board {

    pub fn new() -> Self {
        Board {
            rep: HashMap::new(),
            _prev_rep: HashMap::new(),
            fen_rep: None,
        }
    }

    pub fn get_current(&mut self) -> &mut HashMap<u8, Piece> {
        &mut self.rep
    }

    pub fn get_prev(&mut self) -> &mut HashMap<u8, Piece> {
        &mut self._prev_rep
    }

    pub fn get_fen(&self) -> &Option<String> {
        &self.fen_rep
    }

    pub fn set_fen(&mut self, _fen: &str) {
        self.fen_rep = Some(_fen.to_string());
    }
}
