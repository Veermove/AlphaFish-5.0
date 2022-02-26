pub use std::collections::HashMap;
pub use super::piece::{Piece};

pub struct Board {
    rep: HashMap<u8, Piece>,
    prev_rep: Option<HashMap<u8, Piece>>,
    fen_rep: Option<String>,
    white_to_move: bool,
    en_passant: Option<u8>,
    halfmove_clock: u16,
    fullmove_clock: u16,
    castles: u8,
}

impl Board {

    pub fn to_builder() -> BoardBuilder {
        BoardBuilder::new()
    }

    pub fn new() -> Self {
        Board {
            rep: HashMap::new(),
            prev_rep: Some(HashMap::new()),
            fen_rep: None,
            white_to_move: true,
            en_passant: None,
            halfmove_clock: 0,
            fullmove_clock: 0,
            castles: 0,
        }
    }

    pub fn new_fen(_fen_rep: Option<String>, _rep: HashMap<u8, Piece>, _prev_rep: Option<HashMap<u8, Piece>>,
        _white_to_move: bool, _en_passant: Option<u8>, _halfmove_clock: u16, _fullmove_clock: u16, _castles: u8)
        -> Self {

        Board {
            rep: _rep,
            prev_rep: _prev_rep,
            fen_rep: _fen_rep,
            white_to_move: _white_to_move,
            en_passant: _en_passant,
            halfmove_clock: _halfmove_clock,
            fullmove_clock: _fullmove_clock,
            castles: _castles,
        }
    }

    pub fn get_current_mut(&mut self) -> &mut HashMap<u8, Piece> {
        &mut self.rep
    }

    pub fn get_current(&self) -> &HashMap<u8, Piece> {
        &self.rep
    }

    pub fn get_prev(&mut self) -> &mut Option<HashMap<u8, Piece>> {
        &mut self.prev_rep
    }

    pub fn get_fen(&self) -> &Option<String> {
        &self.fen_rep
    }

    pub fn set_fen(&mut self, _fen: &str) {
        self.fen_rep = Some(_fen.to_string());
    }
}

pub struct BoardBuilder {
    rep_f: Option<HashMap<u8, Piece>>,
    prev_rep_f: Option<Option<HashMap<u8, Piece>>>,
    fen_rep_f: Option<Option<String>>,
    white_to_move_f: Option<bool>,
    en_passant_f: Option<Option<u8>>,
    halfmove_clock_f: Option<u16>,
    fullmove_clock_f: Option<u16>,
    castles_f: Option<u8>
}

impl BoardBuilder {
    pub fn new() -> Self {
        BoardBuilder {
            rep_f: None,
            prev_rep_f: None,
            fen_rep_f: None,
            white_to_move_f: None,
            en_passant_f: None,
            halfmove_clock_f: None,
            fullmove_clock_f: None,
            castles_f: None,
        }

    }

    pub fn build(self) -> Board {
        Board::new_fen(self.fen_rep_f.flatten(),
            self.rep_f.expect("BoardBuilder ex: None 'rep'"),
            self.prev_rep_f.flatten(),
            self.white_to_move_f.unwrap_or(true),
            self.en_passant_f.flatten(),
            self.halfmove_clock_f.unwrap_or(0),
            self.fullmove_clock_f.unwrap_or(0),
            self.castles_f.unwrap_or(0)
        )
    }

    pub fn rep(mut self, _rep: HashMap<u8, Piece>) -> Self {
        self.rep_f = Some(_rep);
        self
    }

    pub fn prev_rep(mut self, _prev_rep: Option<HashMap<u8, Piece>>) -> Self {
        self.prev_rep_f = Some(_prev_rep);
        self
    }

    pub fn fen_rep(mut self, _fen_rep: Option<String>) -> Self {
        self.fen_rep_f = Some(_fen_rep);
        self
    }

    pub fn white_to_move(mut self, _white_to_move: bool) -> Self {
        self.white_to_move_f = Some(_white_to_move);
        self
    }

    pub fn en_passant(mut self, _en_passant: Option<u8>) -> Self {
        self.en_passant_f = Some(_en_passant);
        self
    }

    pub fn halfmove_clock(mut self, _halfmove_clock: u16) -> Self {
        self.halfmove_clock_f = Some(_halfmove_clock);
        self
    }

    pub fn fullmove_clock(mut self, _fullmove_clock: u16) -> Self {
        self.fullmove_clock_f = Some(_fullmove_clock);
        self
    }

    pub fn castles(mut self, _castles: u8) -> Self {
        self.castles_f = Some(_castles);
        self
    }

}
