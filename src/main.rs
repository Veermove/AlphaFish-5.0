mod model;
mod translator;

use model::piece::{Piece, Color, Signature};
use model::offsets::Offsets;
use translator::translations;

fn main() {
    let offsets = Offsets::init();
    let board = translations::fen_to_memory("");
}
