use crate::model::board::{Board, HashMap, Piece};
use crate::model::move_rep::{Move};
use crate::model::offsets::Offsets;
use crate::translator::move_translations::{calc_letters, calc_current};

pub fn get_legal_moves(offsets: Offsets, board: &Board) -> Vec<Move> {
    get_pseudolegal_moves(offsets, board)
}

fn get_pseudolegal_moves(offsets: Offsets, board: &Board) -> Vec<Move> {
    let sliding_moves = board.get_current()
    .values()
    .filter(|piece| (piece.get_color() == 0b10 && board.get_white_to_move())
        || (piece.get_color() != 0b10 && !board.get_white_to_move())
    )
    .map(|piece| {
        if piece.get_figure() >= 3 || piece.get_figure() <= 5 {
            generate_sliding_moves(
                offsets.get(piece.get_id()),
                piece,
                board.get_current()
            )
        } else

    }

    )
    .reduce(|mut accum, mut another| {
        accum.append(&mut another);
        return accum;
    });
    sliding_moves.unwrap()
}

fn generate_sliding_moves(offset: &Vec<i8>, piece: &Piece, other_p: &HashMap<u8, Piece>) -> Vec<Move> {
    let mut generated_moves = Vec::new();
    let (current_col, current_row) = calc_current(piece.get_pos());

    for direction in offset {
        let mut pos = piece.get_pos() as i8;
        loop {
            pos += direction;
            if pos > 63 || pos < 0 {
                break;
            }
            let (col, row) = calc_letters(pos as u8);
            if col > 7 || col < 0 {
                break;
            }
            if row > 8 || row < 1 {
                break;
            }
            let mut pos = pos as u8;
            if other_p.contains_key(&pos) {
                let other = other_p[&pos];
                if other.get_color() == piece.get_color() {
                    break;
                }
            }
            generated_moves.push(Move::to_builder()
                .piece_id(piece.get_id())
                .current_col(Some(current_col))
                .current_row(Some(current_row))
                .target_square(pos)
                .build());
        };
    };
    generated_moves
}
