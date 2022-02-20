mod model;

use model::piece::{Piece, Color, Figure};

fn main() {
    println!("Hello, world!");
    let piece = Piece::new(0b11001, 1, false);
    match piece.get_color() {
        Color::White => println!("white"),
        Color::Black => println!("black"),
    }


}


// mod my_struct;                  <-- Import the module code, placing
//                                     it into the 'my_struct'
//                                     namespace
// use crate::my_struct::MyStruct; <-- Map the fully qualified (from
//                                     the crate root) struct
//                                     declaration to just 'MyStruct'fn main() {
//   let _ms = MyStruct {};        <-- Yay, we found it! .. or did we?
