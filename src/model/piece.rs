use std::fmt::{Display};

// ID: XYZZZ
// X - ALIVE STATE, Y - COLOR, Z - PIECE IDENTITY

// White queen = 10101
// Black pawn  = 11001

// 00011001

// 1 alive, 0 dead
// 0 white, 1 black
// 001 - pawn
// 010 - knight
// 011 - bishop
// 100 - rook
// 101 - queen
// 111 - king

#[derive(PartialEq)]
pub enum Color {
    White,
    Black,
}

pub enum Signature {
    Pawn(char),
    King(char),
    Queen(char),
    Rook(char),
    Knight(char),
    Bishop(char),
}

pub struct Piece {
    id: u8,
    pos: u8,
    has_moved: bool,
}

impl Piece {
    pub fn new(_id: u8, _pos: u8, _has_moved: bool) -> Self {
        Piece {
            id: _id,
            pos: _pos,
            has_moved: _has_moved
        }
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    pub fn get_color_E(&self) -> Color {
        match self.id >> 3 {
            0b11 => Color::Black,
            0b10 => Color::White,
            _    => panic!("Piece.get_color() ex: No such color")
        }
    }

    pub fn get_color(&self) -> u8 {
        self.id >> 3
    }

    pub fn get_figure(&self) -> u8 {
        self.id & 0b00111
    }

    pub fn get_signature(&self) -> Signature {
        match self.id & 0b00111 {
            0b001 => Signature::Pawn(if self.get_color_E() == Color::White { 'P' } else { 'p' }),
            0b010 => Signature::Knight(if self.get_color_E() == Color::White { 'N' } else { 'n' }),
            0b011 => Signature::Bishop(if self.get_color_E() == Color::White { 'B' } else { 'b' }),
            0b100 => Signature::Rook(if self.get_color_E() == Color::White { 'R' } else { 'r' }),
            0b101 => Signature::Queen(if self.get_color_E() == Color::White { 'Q' } else { 'q' }),
            0b111 => Signature::King(if self.get_color_E() == Color::White { 'K' } else { 'k' }),
            _     => panic!("Piece.get_figure() ex: No such figure")
        }
    }

    pub fn get_moved(&self) -> bool {
        self.has_moved
    }

    pub fn get_pos(&self) -> u8 {
        self.pos
    }
}
