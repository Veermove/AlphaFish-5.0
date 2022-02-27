pub use crate::translator::move_translations::calc_sqr;

pub struct Move {
    target_square: u8,
    current_col: Option<u8>,
    current_row: Option<u8>,
    piece_id: u8,
    move_str: Option<String>,
}

impl Move {

    pub fn to_builder() -> MoveBuilder {
        MoveBuilder::new()
    }

    pub fn new_full(target: u8, current_c: Option<u8>, current_r: Option<u8>,
        id: u8, ms: Option<String>) -> Move {
        Move {
            target_square: target,
            current_col: current_c,
            current_row: current_r,
            piece_id: id,
            move_str: ms,
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

    pub fn get_current_sqr(&self) -> Option<u8> {
        self.current_row.zip(self.current_col)
        .map(|(p_ver, p_hor)| calc_sqr(p_ver as i8, p_hor as i8))
        .flatten()
    }

    pub fn tup_full(&self) -> Option<(u8, u8, u8)> {
        self.get_current_sqr()
            .map(|curent| (curent, self.target_square, self.piece_id))
    }

    pub fn tup(&self) -> (u8, u8) {
        (self.target_square, self.piece_id)
    }
}

pub struct MoveBuilder {
    f_target_square: Option<u8>,
    f_current_col: Option<Option<u8>>,
    f_current_row: Option<Option<u8>>,
    f_piece_id: Option<u8>,
    f_move_str: Option<Option<String>>,
}

impl MoveBuilder {
    pub fn new() -> Self {
        MoveBuilder {
            f_target_square: None,
            f_current_col: None,
            f_current_row: None,
            f_piece_id: None,
            f_move_str: None,
        }
    }

    pub fn build(self) -> Move {
        Move::new_full(
            self.f_target_square.expect("MoveBuilder.build(): target sqr not provided"),
            self.f_current_col.flatten(),
            self.f_current_row.flatten(),
            self.f_piece_id.expect("MoveBuilder.build(): pieceId not provided"),
            self.f_move_str.flatten(),
        )
    }

    pub fn target_square(mut self, target: u8) -> Self {
        self.f_target_square = Some(target);
        self
    }

    pub fn current_col(mut self, current_c: Option<u8>) -> Self {
        self.f_current_col = Some(current_c);
        self
    }

    pub fn f_current_row(mut self, current_r: Option<u8>) -> Self {
        self.f_current_row = Some(current_r);
        self
    }

    pub fn piece_id(mut self, id: u8) -> Self {
        self.f_piece_id = Some(id);
        self
    }

    pub fn move_str(mut self, move_str: Option<String>) -> Self {
        self.f_move_str = Some(move_str);
        self
    }
}
