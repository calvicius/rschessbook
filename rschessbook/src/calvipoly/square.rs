use super::util;
use super::color;


pub const EPS_SQUARE: usize = 6;
pub const EMPTY: usize = 7;

/* Some useful squares */
pub const A1: usize = 56;
pub const B1: usize = 57;
pub const C1: usize = 58;
pub const D1: usize = 59;
pub const E1: usize = 60;
pub const F1: usize = 61;
pub const G1: usize = 62;
pub const H1: usize = 63;

pub const A8: usize = 0;
pub const B8: usize = 1;
pub const C8: usize = 2;
pub const D8: usize = 3;
pub const E8: usize = 4;
pub const F8: usize = 5;
pub const G8: usize = 6;
pub const H8: usize = 7;

pub const BRD_SQ_NUM: usize = 64;    // numero de casillas en el tablero
pub const SQUARE_NONE: usize = 99;

pub const RANK1: usize = 0;
pub const RANK2: usize = 1;
pub const RANK3: usize = 2;
pub const RANK4: usize = 3;
pub const RANK5: usize = 4;
pub const RANK6: usize = 5;
pub const RANK7: usize = 6;
pub const RANK8: usize = 7;


/* array of convenience convert square number to 
algebraic notation only for tests and fen conversion*/
pub const ALGEBRA_SQUARE: [&str; 64] = [
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
];


pub const REVERSED: [usize; 64] = [
    56, 57, 58, 59, 60, 61, 62, 63,
    48, 49, 50, 51, 52, 53, 54, 55,
    40, 41, 42, 43, 44, 45, 46, 47,
    32, 33, 34, 35, 36, 37, 38, 39,
    24, 25, 26, 27, 28, 29, 30, 31,
    16, 17, 18, 19, 20, 21, 22, 23,
     8,  9, 10, 11, 12, 13, 14, 15,
     0,  1,  2,  3,  4,  5,  6,  7
];


// square_to_string()

pub fn square_to_string(square: usize, string_: &mut String, size: usize) -> bool {

    assert!(size>=3);
 
    if size < 3 { return false; }
 
    string_.push(('a' as u8 + util::get_col(REVERSED[square]) as u8) as char);
    string_.push(('1' as u8 + util::get_row(REVERSED[square]) as u8) as char);
    true
}


// square_from_string()
 
pub fn square_from_string(string_: &mut String) -> usize {
 
    assert!(!string_.is_empty());

    let fl = string_.chars().nth(0).unwrap();
    let rk = string_.chars().nth(1).unwrap();
    
    if fl < 'a' || fl > 'h' { return SQUARE_NONE; }
    if rk < '1' || rk > '8' { return SQUARE_NONE; }

    let opt = ALGEBRA_SQUARE.iter().position(|&r| r == string_); 
    match opt {
        Some(idx) => {
            return idx;
        },
        None => {
            return SQUARE_NONE;
        },
    }
    //let file: i32 = file_from_char(fl);
    //let rank: i32 = rank_from_char(rk);
 
    //square_make(file, rank)
}


// char_is_rank()

pub fn char_is_rank(c: char) -> bool {

    c >= '1' && c <= '8'
}


// char_is_file()

pub fn char_is_file(c: char) -> bool {

    c >= 'a' && c <= 'h'
}


// square_side_rank()

pub fn square_side_rank(square: usize, colour: usize) -> usize {

    //int rank;
 
    //assert!(square_is_ok(square));
    //assert!(colour::colour_is_ok(colour));
 
    let mut rank = util::get_row(square);
    if color::colour_is_black(colour) { rank = 7-rank; }
 
    return rank;
}


