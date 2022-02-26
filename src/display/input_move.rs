pub use std::io;
pub use crate::model::move_rep::Move;

pub fn input_from_usr(persist: bool, white_to_play: bool) -> Option<Move> {
    match persist {
        true => Some(take_move_persist(white_to_play)),
        false => take_move_yield(white_to_play)
    }
}

fn take_move_persist(white_to_play: bool) -> Move {
    let move_msg = if white_to_play {"White to play: \n"} else {"Black to play: \n"};

    print!("\x1B7"); // save cursor pos
    loop {
        print!("\x1B8"); // move cursor to saved pos
        print!("\x1B[0J"); // erase from cursor to beggining
        print!("{}", move_msg);
        match parse_input(take_move().as_str()) {
            Ok(correct) => correct,
            Err(_) => continue,
        };

    }
}

fn take_move_yield(white_to_play: bool) -> Option<Move> {
    let move_msg = if white_to_play {"White to play: \n"} else {"Black to play: \n"};
    print!("{}", move_msg);
    parse_input(take_move().as_str()).ok()
}


fn take_move() -> String {
    let mut input = String::new();
    let take = io::stdin().read_line(&mut input);
    input
}

fn parse_input(input: &str) -> Result<Move, &str> {
    Err("Not implemented")
}
