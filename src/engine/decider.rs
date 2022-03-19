use crate::model::board::{Board, Piece};
use crate::model::move_rep::Move;


pub fn make_move(game: &mut Board, moving_piece: &mut Piece, given_move: Move,
    captured_piece: Option<&Piece>) -> () {

    if game.get_prev().is_some() {
        game.set_prev(Some(*game.get_current()));
    }
    let mut board = game.get_current_mut();
    if captured_piece.is_some() {
        game.set_halfmove_clock(0);
        board.remove(&captured_piece.unwrap().get_pos());
    }

    let moving_piece = board.remove(&moving_piece.get_pos()).unwrap();
    let target = given_move.get_target_sqr();
    moving_piece.set_pos(&target);
    board.insert(target, moving_piece);
}
