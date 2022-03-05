use std::collections::{HashMap};
use crate::translator::move_translations::{calc_letters};
use crate::model::offsets::Offsets;
use crate::model::move_rep::{Move};
use crate::model::board::{Board, Piece};

pub fn get_legal_moves(offsets: Offsets, board: &Board) -> Vec<Move> {
    get_pseudolegal_moves(offsets, board)
}

fn get_pseudolegal_moves(offsets: Offsets, board: &Board) -> Vec<Move> {
    board.get_current()
    .values()
    .filter(|piece| {
        (piece.get_color() == 0b10 && board.get_white_to_move())
        || (piece.get_color() != 0b10 && !board.get_white_to_move())
    })
    .map(|piece| {

        if piece.get_figure() >= 0b011 && piece.get_figure() <= 0b101 {
            // BISHOP, ROOK, QUEEN
            generate_sliding_moves(offsets.get(piece.get_figure()), piece, board.get_current())
        } else if piece.get_figure() == 0b010 {
            // KNIGHT
            generate_king_knight_moves(offsets.get(piece.get_figure()), piece, board.get_current())
        } else if piece.get_figure() == 0b111 {
            // KING
            let mut king_moves = generate_king_knight_moves(offsets.get(piece.get_figure()), piece, board.get_current());
            king_moves.append(&mut generate_castles_moves(piece, board.get_current()));
            king_moves
        } else {
            // PAWN
            generate_pawn_moves(offsets.get(piece.get_figure()), piece, board.get_current(), board.get_en_passant())
        }
    })
    .reduce(|mut accum, mut another| {
        accum.append(&mut another);
        return accum;
    })
    .unwrap()
}

fn generate_pawn_moves(offset: &Vec<(i8, i8, i8)>, piece:
    &Piece, other_p: &HashMap<u8, Piece>,
    en_passant: Option<u8>) -> Vec<Move> {
    let mut pawn_moves = Vec::new();
    let col_row = calc_letters(piece.get_pos());
    let col_row = (col_row.0 as i8, col_row.1 as i8);
    let mut off_itr = offset.iter();

    { // EN PASSANT
        if en_passant.is_some() {
            pawn_moves.push(Move::to_builder()
                .piece_id(piece.get_id())
                .current_col(Some(col_row.0 as u8))
                .current_row(Some(col_row.1 as u8))
                .target_square(en_passant.unwrap())
                .build());
        }
    };

    { // MOVE TWO SPACES UP
        let (c_off, r_off, off) = off_itr.next().unwrap();
        let (_, _, pos) = (col_row.0 + c_off, col_row.1 + r_off, (piece.get_pos() as i8) + off);
        if !piece.get_has_moved() {
            pawn_moves.push(Move::to_builder()
                .piece_id(piece.get_id())
                .current_col(Some(col_row.0 as u8))
                .current_row(Some(col_row.1 as u8))
                .target_square(pos as u8)
                .build());
        }
    };

    { // MOVE ONE SPACE UP
        let (c_off, r_off, off) = off_itr.next().unwrap();
        let (_, row, pos) = (col_row.0 + c_off, col_row.1 + r_off, ((piece.get_pos() as i8) + off) as u8);
        let mut valid_target_sqr = None;

        if other_p.contains_key(&pos) {
            let other = &other_p[&pos];
            if other.get_color() != piece.get_color() {
                valid_target_sqr = Some(pos);
            }
        } else {
            valid_target_sqr = Some(pos);
        }
        if valid_target_sqr.is_some() {
            pawn_moves.push(Move::to_builder()
                .piece_id(piece.get_id())
                .current_col(Some(col_row.0 as u8))
                .current_row(Some(col_row.1 as u8))
                .target_square(valid_target_sqr.unwrap())
                .special_move(check_for_promotion(row, piece))
                .build());
        }
    };

    { // MOVE DIAGONALLY
        for (c_off, r_off, off) in off_itr {
            let (col, row, pos) = (col_row.0 + c_off, col_row.1 + r_off, ((piece.get_pos() as i8) + off) as u8);
            if other_p.contains_key(&pos) {
                let other = &other_p[&pos];
                if other.get_color() == piece.get_color() {
                    continue;
                }
            } else {
                continue;
            }
            if check_bounds(col, row, pos as i8) {
                pawn_moves.push(Move::to_builder()
                    .piece_id(piece.get_id())
                    .current_col(Some(col_row.0 as u8))
                    .current_row(Some(col_row.1 as u8))
                    .target_square(pos)
                    .special_move(check_for_promotion(row, piece))
                    .build());
            }

        }
    };
    pawn_moves
}

fn check_for_promotion(row: i8, piece: &Piece) -> bool {
     (row == 8 && piece.get_color() == 0b10 ) || (row == 1 && piece.get_color() == 0b11 )
}

pub fn check_bounds(col: i8, row: i8, pos: i8) -> bool {
    if pos > 63 || pos < 0 {
        return false;
    }
    if col > 7 || col < 0 {
        return false;
    }
    if row > 8 || row < 1 {
        return false;
    }
    true
}

fn generate_king_knight_moves(offset: &Vec<(i8, i8, i8)>, piece: &Piece, other_p: &HashMap<u8, Piece>) -> Vec<Move> {
    let mut knight_moves = Vec::new();
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

        let pos = pos as u8;
        if other_p.contains_key(&pos) {
            let other = &other_p[&pos];
            if other.get_color() == piece.get_color() {
                continue;
            }
        }
        knight_moves.push(Move::to_builder()
            .piece_id(piece.get_id())
            .current_col(Some(col_row.0 as u8))
            .current_row(Some(col_row.1 as u8))
            .target_square(pos)
            .build());

    }

    knight_moves
}

fn generate_sliding_moves(offset: &Vec<(i8, i8, i8)>, piece: &Piece, other_p: &HashMap<u8, Piece>) -> Vec<Move> {
    let mut generated_moves = Vec::new();
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

            let pos = pos as u8;
            if other_p.contains_key(&pos) {
                let other = &other_p[&pos];
                if other.get_color() == piece.get_color() {
                    break;
                }
            }
            generated_moves.push(Move::to_builder()
                .piece_id(piece.get_id())
                .current_col(Some(col_row.0 as u8))
                .current_row(Some(col_row.1 as u8))
                .target_square(pos)
                .build());
        };
    };
    generated_moves
}

fn generate_castles_moves(piece: &Piece, other_p: &HashMap<u8, Piece>) -> Vec<Move> {
    let mut castles = Vec::new();
    if piece.get_has_moved() {
        return castles;
    }

    let (left_rook_s, right_rook_s): (u8, u8) = if piece.get_color() == 0b10 { (0, 7) } else { (56, 63) };
    let (col, row) = calc_letters(piece.get_pos());
    { // Castle left
        let (d_sqr, c_sqr) = if piece.get_color() == 0b10 { (2, 3) } else { (58, 59) };
        let left_rook = other_p.get(&left_rook_s);
        if left_rook.is_some() && !other_p.contains_key(&d_sqr) && !other_p.contains_key(&c_sqr) {
            if !left_rook.unwrap().get_has_moved() {
                castles.push(Move::to_builder()
                    .piece_id(piece.get_id())
                    .current_col(Some(col))
                    .current_row(Some(row))
                    .target_square(if piece.get_color() == 0b10 { 2 } else { 58 })
                    .special_move(true)
                    .build());
            }
        }
    };

    { // Castle right
        let right_rook = other_p.get(&right_rook_s);
        let (d_sqr, c_sqr) = if piece.get_color() == 0b10 { (5, 6) } else { (62, 61) };
        if right_rook.is_some() {
            if right_rook.is_some() && !other_p.contains_key(&d_sqr) && !other_p.contains_key(&c_sqr) {
                castles.push(Move::to_builder()
                    .piece_id(piece.get_id())
                    .current_col(Some(col))
                    .current_row(Some(row))
                    .target_square(if piece.get_color() == 0b10 { 6 } else { 62 })
                    .special_move(true)
                    .build());
            }
        }
    };

    castles
}
