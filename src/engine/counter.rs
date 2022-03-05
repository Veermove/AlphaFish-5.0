use std::collections::hash_set::IntoIter;
use std::collections::{HashMap, HashSet};
use crate::translator::move_translations::{calc_letters};
use crate::model::piece::Piece;
use crate::model::offsets::Offsets;
use crate::engine::policeman::check_bounds;

pub fn get_attacked_sqrs(is_white: bool, pieces:&HashMap<u8,Piece>, offsets: Offsets) -> HashSet<u8> {
    pieces.values()
    .filter(|piece| {
        (piece.get_color() == 0b10 && is_white)
        || (piece.get_color() != 0b10 && !is_white)
    })
    .flat_map(|piece| {
        if piece.get_figure() >= 0b011 && piece.get_figure() <= 0b101 {
            // BISHOP, ROOK, QUEEN
            sliding_attacks(piece, offsets.get(piece.get_figure()), pieces)
        } else if piece.get_figure() == 0b010 {
            // KNIGHT
            king_knight_attack(piece, offsets.get(piece.get_figure()))
        } else if piece.get_figure() == 0b111 {
            // KING
            king_knight_attack(piece, offsets.get(piece.get_figure()))
        } else {
            // PAWN
            pawn_attack(piece)
        }
    })
    .collect()
}

fn pawn_attack(piece: &Piece) -> IntoIter<u8> {
    let col = piece.get_pos() & 0b111;
    let mut attacked_sqrs = HashSet::new();
    // attack left
    if col > 0 {
        attacked_sqrs.insert(piece.get_pos() + 7);
    }

    // attack right
    if col + 1 <= 7 {
        attacked_sqrs.insert(piece.get_pos() + 9);
    }

    attacked_sqrs.into_iter()
}

fn king_knight_attack(piece: &Piece, offset: &Vec<(i8, i8, i8)>) -> IntoIter<u8> {
    let mut attacked_sqrs = HashSet::new();
    let col_row = calc_letters(piece.get_pos());
    let col_row = (col_row.0 as i8, col_row.1 as i8);

    for (c_off, r_off, off) in offset {
        let mut pos = piece.get_pos() as i8;
        let (mut col, mut row) = col_row.clone();
        pos += off;
        col += c_off;
        row += r_off;

        if !check_bounds(col, row, pos) {
            continue;
        }
        attacked_sqrs.insert(pos as u8);
    }
    attacked_sqrs.into_iter()
}

fn sliding_attacks(piece: &Piece, offset: &Vec<(i8, i8, i8)>, pieces: &HashMap<u8, Piece>) -> IntoIter<u8> {
    let mut attacked_sqrs = HashSet::new();
    let col_row = calc_letters(piece.get_pos());
    let col_row = (col_row.0 as i8, col_row.1 as i8);

    for (c_off, r_off, off) in offset {
        let mut pos = piece.get_pos() as i8;
        let (mut col, mut row) = col_row.clone();
        loop {
            pos += off;
            col += c_off;
            row += r_off;

            if !check_bounds(col, row, pos) {
                break;
            }

            attacked_sqrs.insert(pos as u8);

            if pieces.contains_key(&(pos as u8)) {
                break;
            }

        }
    }
    attacked_sqrs.into_iter()
}
