mod translator;
mod model;
mod engine;
mod display;

use translator::{fen_translations, move_translations};
use model::piece::{Piece, Color, Signature};
use model::offsets::Offsets;
use model::move_rep::{Move};
use engine::policeman;
use engine::counter;
use display::{output, input_move};


fn main() {

    let start_pos = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let offsets = Offsets::init();
    // let board = fen_translations::fen_to_memory("1b7/8/8/4K3/8/8/8/8 w - f6 0 1");
    let board = fen_translations::fen_to_memory(start_pos);
    output::show_board(&board);

    // let given = input_move::input_from_usr(true, true).unwrap();
    // let (a, b) = given.tup();
    // println!("move was T: {}, id: {:b}", a, b);
    // let moves = policeman::get_legal_moves(offsets, &board);
    // for mo in moves {
    //     println!("{}", mo.to_str().as_str());
    // }

    for sqr in counter::get_attacked_sqrs(false, board.get_current(), offsets) {
        let (r, c) = move_translations::calc_letters_elegant(sqr);
        println!("{}{}", r, c);
    }
}
