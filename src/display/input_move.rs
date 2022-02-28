pub use std::io;
pub use crate::model::move_rep::Move;
pub use crate::translator::move_translations::parse;
pub use std::string::String;

pub fn input_from_usr(persist: bool, white_to_play: bool) -> Option<Move> {
    match persist {
        true => Some(take_move_persist(white_to_play)),
        false => take_move_yield(white_to_play)
    }
}

fn take_move_persist(white_to_play: bool) -> Move {
    let move_msg = if white_to_play {"White to play: \n"} else {"Black to play: \n"};
    // let fail_move_msg = "Illegal move: ".to_owned();
    let mut fail: Option<String> = None;
    print!("\x1B7"); // save cursor pos

    loop {
        print!("\x1B8"); // move cursor to saved pos
        print!("\x1B[0J"); // erase from cursor to beggining
        print!("{}\n{}", fail.unwrap_or("".to_string()), move_msg);
        match parse(take_move().as_str(), white_to_play) {
            Ok(correct) => return correct,
            Err(err) => {
                fail = Some(err.to_string());
                continue
            },
        };

    }
}

fn take_move_yield(white_to_play: bool) -> Option<Move> {
    let move_msg = if white_to_play {"White to play: \n"} else {"Black to play: \n"};
    print!("{}", move_msg);
    parse(take_move().as_str(), white_to_play).ok()
}


fn take_move() -> String {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input
}
