use crate::model::move_rep::Move;

pub fn parse(input: &str, input_from_white: bool) -> Result<Move, &str> {
    let mut pos_chars = input.chars().rev();
    pos_chars.next();
    let square = pos_chars.next()
        .zip(pos_chars.next())
        .map(|(p_ver, p_hor)| cord_to_sqr(p_ver, p_hor))
        .flatten();

    let mut piece: Option<char> = None;
    let mut current_c: Option<u8> = None;
    let mut current_r: Option<u8> = None;

    loop {
        let letter = match pos_chars.next() {
            None => break,
            Some(k) => k,
        };

        if letter.is_numeric() {
            let row = (((letter as i8) - 48) - 1) * 8;
            current_r = if row >= 0 && row < 56 { Some(row as u8) } else { None };
        }
        else if letter.is_uppercase() {
            piece = Some(letter)
        }
        else if letter.is_lowercase() {
            let column = (letter as i8) - 97;
            current_c = match letter {
                'a' => Some(column as u8),
                'b' => Some(column as u8),
                'c' => Some(column as u8),
                'd' => Some(column as u8),
                'e' => Some(column as u8),
                'f' => Some(column as u8),
                'g' => Some(column as u8),
                'h' => Some(column as u8),
                _ => None,
            };
        }
    };

    let piece = calc_piece_sign_from_letter(piece, input_from_white);
    if piece.is_none() {
        return Err("Err: Incorrect piece provided. ");
    }
    if square.is_none() {
        return Err("Err: Incorrect target sqr provided. ");
    }

    Ok(
        Move::to_builder()
            .target_square(square.unwrap())
            .f_current_row(current_r)
            .current_col(current_c)
            .piece_id(piece.unwrap())
            .move_str(Some(input.to_string()))
            .build()
    )
}

pub fn calc_piece_sign_from_letter(letter: Option<char>, input_from_white: bool) -> Option<u8> {
    let id = if input_from_white { 0b10000 } else { 0b11000 };
    match letter {
        None => Some(id + 0b001),
        Some(l) => {
            match l {
                'N' => Some(id + 0b010),
                'B' => Some(id + 0b011),
                'R' => Some(id + 0b100),
                'Q' => Some(id + 0b101),
                'K' => Some(id + 0b111),
                _ => None
            }
        }

    }
}

pub fn cord_to_sqr(p_ver: char, p_hor: char) -> Option<u8> {
    if !p_ver.is_numeric() || !p_hor.is_lowercase() {
        return None;
    }

    let column = (p_hor as i8) - 97;
    let row = (((p_ver as i8) - 48) - 1) * 8;
    calc_sqr(column, row)
}

pub fn calc_sqr(col: i8, row: i8) -> Option<u8> {
    let sum = col + row;
    if sum >= 0 && sum < 64 {
        Some(sum as u8)
    } else {
        None
    }
}
