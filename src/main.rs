mod model;
mod translator;
mod display;
mod engine;

use model::piece::{Piece, Color, Signature};
use model::offsets::Offsets;
use model::move_rep::{Move};
use translator::{fen_translations, move_translations};
use display::{output, input_move};
use engine::policeman;


fn main() {

    let start_pos = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let offsets = Offsets::init();
    let board = fen_translations::fen_to_memory("8/k7/8/4Pp2/8/8/8/K7 w - f6 0 1");
    output::show_board(&board);

    let given = input_move::input_from_usr(true, true).unwrap();
    let (a, b) = given.tup();
    println!("move was T: {}, id: {:b}", a, b);
    let moves = policeman::get_legal_moves(offsets, &board);
    for mo in moves {
        println!("{}", mo.to_str().as_str());
    }
}
