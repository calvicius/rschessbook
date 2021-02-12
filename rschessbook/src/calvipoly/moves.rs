use super::util;
use super::square;

/* For move generation */
pub const MOVE_TYPE_NONE: usize     = 0;
pub const MOVE_TYPE_NORMAL: usize   = 1;
pub const MOVE_TYPE_CASTLE: usize   = 2;
pub const MOVE_TYPE_PAWN_TWO: usize = 3;
pub const MOVE_TYPE_EPS: usize      = 4;

pub const MOVE_TYPE_PROMOTION_TO_QUEEN: usize  = 5;
pub const MOVE_TYPE_PROMOTION_TO_ROOK: usize   = 6;
pub const MOVE_TYPE_PROMOTION_TO_BISHOP: usize = 7;
pub const MOVE_TYPE_PROMOTION_TO_KNIGHT: usize = 8;

// flags for polyglot
pub const PROMOTION_POLY_NONE: usize   = 0;
pub const PROMOTION_POLY_KNIGHT: usize = 1;
pub const PROMOTION_POLY_BISHOP: usize = 2;
pub const PROMOTION_POLY_ROOK: usize   = 3;
pub const PROMOTION_POLY_QUEEN: usize  = 4;

// a1a1 cannot be a legal move
//pub const MOVE_NONE: usize = 0;

/*
* "promotion piece" is encoded in polyglot as follows
*
* none       0
* knight     1
* bishop     2
* rook       3
* queen      4
*
* So in board module I'll do in gen_push method:alloc
*   if SMove.tipe >= MOVE_TYPE_PROMOTION_TO_QUEEN {
*       SMove.prom_zob = 4;
*   }
*/




/* For castle rights we use a bitfield, like in TSCP
 *
 * 0001 -> White can short castle
 * 0010 -> White can long castle
 * 0100 -> Black can short castle
 * 1000 -> Black can long castle
 *
 * 15 = 1111 = 1*2^3 + 1*2^2 + 1*2^1 + 1*2^0
 *
 */
//pub static mut CASTLE_RIGHTS: u16 = 15;		/* At start position all castle types ar available */


/* This mask is applied like this
 *
 * castle &= castle_mask[from] & castle_mask[dest]
 *
 * When from and dest are whatever pieces, then nothing happens, otherwise
 * the values are chosen in such a way that if vg the white king moves
 * to F1 then
 *
 * castle = castle & (12 & 15)
 * 1111 & (1100 & 1111) == 1111 & 1100 == 1100
 *
 * and white's lost all its castle rights
 *
 * */

/* another way is:
 *
    let c_long_b = 8;	// 1*2^3
	let c_short_b = 4;	// 1*2^2
	let c_long_w = 2;	// 1*2^1
	let c_short_w = 1;	// 1*2^0
	let mut rights = c_long_b | c_short_b | c_long_w | c_short_w;
	
	println!("{:#06b} , {:#06b} , {:#06b}, {:#06b}", c_long_b, c_short_b, c_long_w, c_short_w);
	println!("{:#06b}", rights);
	
	//el negro se enroca y pierde los dos derechos
	//rights &= (1+2);
	//println!("{:#06b} - {:?}", rights, isize::from_str_radix("1100", 2).unwrap());
	
	// el blanco se enroca y pierde los derechos
	//rights &= (8+4);
    //println!("{:#06b} -", rights);
    
	// el blanco solo pierde el enroque corto
	//rights &= (8+4+2);
	//println!("{:#06b} -", rights);
	
	// el blanco solo pierde el enroque largo
	//rights &= (8+4+1);
	//println!("{:#06b} -", rights);
	
	// el negro solo pierde el enroque largo
	//rights &= (4+2+1);
	//println!("{:#06b} -", rights);
	
	// el negro solo pierde el enroque corto
	rights &= (8+2+1);
	println!("{:#06b} -", rights);
 *
*/ 


pub const CASTLE_MASK: [usize; 64] = [
     7, 15, 15, 15,  3, 15, 15, 11,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    13, 15, 15, 15, 12, 15, 15, 14
];

/*
pub const CASTLE_MASK: [usize; 64] = [
    13, 15, 15, 15, 12, 15, 15, 14,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
     7, 15, 15, 15,  3, 15, 15, 11
];
*/

// Los enroques WKCA = W_hite K_ing CA_stle
pub enum Enroques { WKCA = 1, WQCA = 2, BKCA = 4, BQCA = 8 } // b'1000 =8, 0100 =4, etc





/* A move is defined by its origin and final squares,
 * the castle rights and the kind of move it's: 
 * normal, enpasant... */
 #[derive(Clone)]
pub struct Smove {
    pub from: usize,
    pub dest: usize,
    //      int castle;
    pub tipe: usize,
    pub promotion_poly: usize,
    pub encoded_move: u16,      /* the format of polyglot */
    pub hash: u64,
}

impl Smove {
    pub fn new() -> Self {
        Smove {
            from: 0,
            dest: 0,
            tipe: MOVE_TYPE_NONE,
            promotion_poly: PROMOTION_POLY_NONE,
            encoded_move: 0,
            hash: 0,
        }
    }

    pub fn encode_move(&mut self) {
        let new_king_square: usize;

        let mut to_file = util::get_col(square::REVERSED[self.dest]);
        let mut to_row = util::get_row(square::REVERSED[self.dest]);
        let from_file = util::get_col(square::REVERSED[self.from]);
        let from_row = util::get_row(square::REVERSED[self.from]);
        let promote: usize;

        match self.tipe {
            MOVE_TYPE_CASTLE => {
                // we change the destination of king (polyglot)
                // i.e.: e1g1 -> e1h1
                match self.dest {
                    square::C8 => new_king_square = square::A8,     // black castles long
                    square::G8 => new_king_square = square::H8,     // black castles short
                    square::G1 => new_king_square = square::H1,     // white castles short
                    square::C1 => new_king_square = square::A1,     // white castles long
                    _ => new_king_square = 0,
                }

                to_file = util::get_col(square::REVERSED[new_king_square]);
                to_row = util::get_row(square::REVERSED[new_king_square]);
                promote = PROMOTION_POLY_NONE;
            },
            MOVE_TYPE_PROMOTION_TO_QUEEN => {
                promote = PROMOTION_POLY_QUEEN;
            },
            MOVE_TYPE_PROMOTION_TO_ROOK => {
                promote = PROMOTION_POLY_ROOK;
            },
            MOVE_TYPE_PROMOTION_TO_BISHOP => {
                promote = PROMOTION_POLY_BISHOP;
            },
            MOVE_TYPE_PROMOTION_TO_KNIGHT => {
                promote = PROMOTION_POLY_KNIGHT;
            },
            _ => promote = PROMOTION_POLY_NONE,
        }

        let to_file_bin = to_file as u16;
        let to_row_bin = (to_row << 3) as u16;
        let from_file_bin = (from_file << 6) as u16;
        let from_row_bin = (from_row << 9) as u16;
        let promote_bin = (promote << 12) as u16;

        let mov: u16 = promote_bin | from_row_bin | from_file_bin |
                        to_row_bin | to_file_bin;
        self.encoded_move = mov;
    }
}



/* For storing all moves of game */
#[derive(Clone)]
pub struct Shist {
    pub m: Smove,
    pub castle: usize,
    pub cap: usize,
}

impl Shist {
    pub fn new() -> Self {
        Shist {
            m: Smove::new(),
            castle: 0,
            cap: 0,
        }
    }
}


/* it is almost the position after last valid move */
#[derive(Clone)]
pub struct SundoMove {
    pub square_pieces: [usize; 64],
    pub square_colors: [usize; 64],
    pub side: usize,
    pub castles: usize,
    pub passant: i16,
    pub hash: u64,
}

impl SundoMove {
    pub fn new() -> Self {
        SundoMove {
            square_pieces: [0; 64],
            square_colors: [0; 64],
            side: 0,
            castles: 0,
            passant: -1,
            hash: 0,
        }
    }
}


pub fn get_uci_format (mov: u16) -> String {

	let mut move_s: String = String::new();
	let mut c: char;
	let promote_pieces: [char; 5]= [' ', 'n', 'b', 'r', 'q'];   //" nbrq"
	
	// columna inicial
	let f = (mov>>6)&0o77;
	let ff = f&0x7;
	c = (ff as u8 + 'a' as u8) as char;
	move_s.push(c);
	
	// fila inicial
	let fr = (f>>3) & 0x7;
	c = (fr as u8 + '1' as u8) as char;
	move_s.push(c);
	
	// columna destino
	let t = mov & 0o77;
	let tf = t & 0x7;
	c = (tf as u8 + 'a' as u8) as char;
	move_s.push(c);
	
	// fila destino
	let tr = (t>>3) & 0x7;
	c = (tr as u8 + '1' as u8) as char;
	move_s.push(c);
	
	// promocion
	let p = (mov>>12)&0x7;
	if p != 0 {
        move_s.push(promote_pieces[p as usize]);
    }else{
        move_s.push(' ');
    }
	move_s
}