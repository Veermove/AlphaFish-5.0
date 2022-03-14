pub use crate::model::board::{Board, Piece, HashMap};
pub use crate::model::piece::{Signature, Color};

pub fn show_board(board: &Board) {

    let board_ref = board.get_current();

    let fen = board.get_fen()
        .as_ref()
        .map(|borrowed| borrowed.as_str())
        .unwrap_or("No fen avaiable");

    let rim = "+---+---+---+---+---+---+---+---+";
    let space = "| ";

    let mut board_itr = 72;
    print!("\x1B[H"); // move cursor to (0, 0)
    print!("\x1B[2J"); // erase from cursor to beggining
    println!("FEN string: ");
    println!("{}\n", fen);

    println!("{}", rim);
    for n in 0..8 {
        board_itr -= 16;
        for _ in 0..8 {
            print!("{}", space);
            print!("{}", get_letter(&board_itr, board_ref));
            print!(" ");
            board_itr += 1;
        }
        print!("| {}\n", 8 - n);
        print!("{}\n", rim);

    }
    print!("  a   b   c   d   e   f   g   h   \n\n");
    print!("Full move clock: {}\n", board.get_fullmove_clock());
    print!("Halfmove clock: {}\n\n", board.get_halfmove_clock());

}

pub fn get_letter(field_num: &u8, board_ref: &HashMap<u8, Piece>) -> char {
    // print!("{}", field_num);
    board_ref.get(field_num)
    .map(|piece| (piece.get_figure(), piece.get_color_E()))
    .map(|(fig_id, fig_col)| {
        match fig_id {
            0b001 => (if fig_col == Color::White { 'P' } else { 'p' }),
            0b010 => (if fig_col == Color::White { 'N' } else { 'n' }),
            0b011 => (if fig_col == Color::White { 'B' } else { 'b' }),
            0b100 => (if fig_col == Color::White { 'R' } else { 'r' }),
            0b101 => (if fig_col == Color::White { 'Q' } else { 'q' }),
            0b111 => (if fig_col == Color::White { 'K' } else { 'k' }),
            _     => panic!("Piece.get_figure() ex: No such figure")
        }
    })
    .unwrap_or(' ')
}
