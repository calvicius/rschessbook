use super::{color};


pub const PAWN: usize = 0;
pub const KNIGHT: usize = 1;
pub const BISHOP: usize = 2;
pub const ROOK: usize = 3;
pub const QUEEN: usize = 4;
pub const KING: usize = 5;


pub const BLACK_PAWN: usize   = 0;
pub const WHITE_PAWN: usize   = 1;
pub const BLACK_KNIGHT: usize = 2;
pub const WHITE_KNIGHT: usize = 3;
pub const BLACK_BISHOP: usize = 4;
pub const WHITE_BISHOP: usize = 5;
pub const BLACK_ROOK: usize   = 6;
pub const WHITE_ROOK: usize   = 7;
pub const BLACK_QUEEN: usize  = 8;
pub const WHITE_QUEEN: usize  = 9;
pub const BLACK_KING: usize   = 10;
pub const WHITE_KING: usize   = 11;
pub const POLY_NON_PIECE: usize = 12;



pub fn to_piece_poly(piece: usize, color: usize) -> usize {
    let mut poly_piece: usize = POLY_NON_PIECE;

    assert!(piece <= KING && color <= color::BLACK);

    match piece {
        PAWN => {
            if color == color::WHITE { poly_piece = WHITE_PAWN; }
            else if color == color::BLACK { poly_piece = BLACK_PAWN; }
        },
        KNIGHT => {
            if color == color::WHITE { poly_piece = WHITE_KNIGHT; }
            else if color == color::BLACK { poly_piece = BLACK_KNIGHT; }
        },
        BISHOP => {
            if color == color::WHITE { poly_piece = WHITE_BISHOP; }
            else if color == color::BLACK { poly_piece = BLACK_BISHOP; }
        },
        ROOK => {
            if color == color::WHITE { poly_piece = WHITE_ROOK; }
            else if color == color::BLACK { poly_piece = BLACK_ROOK; }
        },
        QUEEN => {
            if color == color::WHITE { poly_piece = WHITE_QUEEN; }
            else if color == color::BLACK { poly_piece = BLACK_QUEEN; }
        },
        KING => {
            if color == color::WHITE { poly_piece = WHITE_KING; }
            else if color == color::BLACK { poly_piece = BLACK_KING; }
        },
        _ => { poly_piece = POLY_NON_PIECE },
    };

    return poly_piece;
}



// char_is_piece()

pub fn char_is_piece(c: char) -> bool {
    
    "PNBRQK".contains(c)

}