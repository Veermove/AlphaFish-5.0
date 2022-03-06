
use super::{counter, policeman};
use crate::model::board::{Board};
use crate::model::offsets::Offsets;
use crate::model::move_rep::Move;


pub fn get_legal_moves(offsets: &Offsets, board: &Board) -> Vec<Move> {
    let a = 1;
    let b = counter::get_attacked_sqrs(board.get_white_to_move(), board.get_current(), offsets);
    policeman::get_pseudolegal_moves(offsets, board)
    .into_iter();
}

pub fn is_move_legal(board: &mut Board, given_move: Move) -> bool {

}

fn mock_move(board: &mut Board, given_move: Move, f: impl Fn(&Board) -> bool) -> bool {
    let sqr = given_move.get_current_sqr().unwrap();
    if !board.get_current().contains_key(&sqr) {
        return false;
    }

    // MAKE CHANGES TO BOARD
    let mut moving_piece = board.get_current().remove(&sqr).unwrap();
    let prev_state = (moving_piece.get_id(), moving_piece.get_pos());

}
