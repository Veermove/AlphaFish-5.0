pub use crate::model::board::{Board, BoardBuilder, Piece, HashMap};
pub use crate::translator::move_translations::cord_to_sqr;
use crate::model::offsets::OriginalSqrs;

pub fn fen_to_memory(_fen: &str) -> Board {
    let fen_first_class = _fen.to_string().clone();
    let mut fen_iter = fen_first_class.split_whitespace();
    Board::to_builder()
        .rep(extract_rep(fen_iter.next()
            .expect("Translator ex: Incorrect FEN-String - missing board representation")))
        .white_to_move(extract_white_to_move(fen_iter.next().unwrap_or("w")))
        .castles(extract_castles(fen_iter.next().unwrap_or("-")))
        .en_passant(extract_en_passant(fen_iter.next().unwrap_or("-")))
        .halfmove_clock(extract_move_clock(fen_iter.next().unwrap_or("0")))
        .fullmove_clock(extract_move_clock(fen_iter.next().unwrap_or("0")))
        .fen_rep(Some(_fen.to_string()))
        .build()
}

fn extract_en_passant(_given: &str) -> Option<u8> {
    if _given == "-" {
        None
    } else {
        let mut given_itr = _given.chars();
        cord_to_sqr(given_itr.next().unwrap_or('Z'), given_itr.next().unwrap_or('Z'))
    }
}

fn extract_rep(_rep: &str) -> HashMap<u8, Piece> {
    let orig_sqrs = OriginalSqrs::init();
    let mut board_itr: u8 = 56;
    let mut rep = HashMap::new();
    for current in _rep.chars() {
        if current == '/' {
            board_itr -= board_itr % 8;
            board_itr -= 16;
            continue;
        }
        if current.is_numeric() {
            board_itr += current.to_digit(10).expect("Incorrect FEN: Nan") as u8;
            continue;
        }

        let mut _id = 0b10000;
        match current.to_ascii_uppercase() {
            'P' => _id += 0b001,
            'N' => _id += 0b010,
            'B' => _id += 0b011,
            'R' => _id += 0b100,
            'Q' => _id += 0b101,
            'K' => _id += 0b111,
             _  => panic!("Incorrect FEN: ex while extracting rep")
        }

        if current.is_ascii_lowercase() {
            _id += 0b1000
        }
        rep.insert(board_itr, Piece::new(_id, board_itr, orig_sqrs.get_by_sqr(board_itr) != _id));
        board_itr += 1;
    }
    rep
}

fn extract_white_to_move(_fen: &str) -> bool {
    match _fen {
        "w" => true,
         _  => false,
    }
}

fn extract_castles(_fen: &str) -> u8 {
    let mut castles = 0;
    for castle_char in _fen.chars() {
        match castle_char {
            'K' => castles += 0b1000,
            'Q' => castles +=  0b100,
            'k' => castles +=   0b10,
            'q' => castles +=    0b1,
            '-' => castles +=    0b0,
             _  => panic!("Incorrect FEN: ex while extracting castles")
        }
    }
    castles
}

fn extract_move_clock(_fen: &str) -> u16 {
    _fen.parse::<u16>().unwrap_or(0)
}
