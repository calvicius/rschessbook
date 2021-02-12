use super::piece as pi;
use super::square;
use super::color;
use super::moves as mv;
use super::util;
use super::zobrist as zob;


/* Board representation */
#[derive(Clone)]
pub struct Sboard {
    pub piece: [usize; 64],            /* Piece in each square */
    pub color: [usize; 64],            /* Color of each square */
    pub side: usize,                   /* Side to move, value = BLACK or WHITE */
    hist: Vec<mv::Shist>,       /* Game length < 6000 */
    hdp: usize, 			    /* Current move order used as index of hist*/
    count_make_move: usize,
    pub castle_rights: usize,
    ply: usize,                 /* half moves */
    ply_pawn: usize,            /* since last pawn move */
    pub en_passant: i16,
    pub hash_key: u64,
}

impl Sboard {
    pub fn new() -> Self {
        
        let init_pos = [
            pi::ROOK, pi::KNIGHT, pi::BISHOP, pi::QUEEN, pi::KING, pi::BISHOP, pi::KNIGHT, pi::ROOK,
            pi::PAWN, pi::PAWN, pi::PAWN, pi::PAWN, pi::PAWN, pi::PAWN, pi::PAWN, pi::PAWN,
            square::EMPTY, square::EMPTY, square::EMPTY, square::EMPTY,
                square::EMPTY, square::EMPTY, square::EMPTY, square::EMPTY,
            square::EMPTY, square::EMPTY, square::EMPTY, square::EMPTY,
                square::EMPTY, square::EMPTY, square::EMPTY, square::EMPTY,
            square::EMPTY, square::EMPTY, square::EMPTY, square::EMPTY,
                square::EMPTY, square::EMPTY, square::EMPTY, square::EMPTY,
            square::EMPTY, square::EMPTY, square::EMPTY, square::EMPTY,
                square::EMPTY, square::EMPTY, square::EMPTY, square::EMPTY,
            pi::PAWN, pi::PAWN, pi:: PAWN, pi:: PAWN, pi::PAWN, pi::PAWN, pi::PAWN, pi::PAWN, 
            pi::ROOK, pi::KNIGHT, pi::BISHOP, pi::QUEEN, pi::KING, pi::BISHOP, pi::KNIGHT, pi::ROOK
        ];
        let init_colour = [
            color::BLACK, color::BLACK, color::BLACK, color::BLACK, 
                color::BLACK, color::BLACK, color::BLACK, color::BLACK,
            color::BLACK, color::BLACK, color::BLACK, color::BLACK, 
                color::BLACK, color::BLACK, color::BLACK, color::BLACK,
            square::EMPTY, square::EMPTY, square::EMPTY, square::EMPTY, 
                square::EMPTY, square::EMPTY, square::EMPTY, square::EMPTY,
            square::EMPTY, square::EMPTY, square::EMPTY, square::EMPTY, 
                square::EMPTY, square::EMPTY, square::EMPTY, square::EMPTY,
            square::EMPTY, square::EMPTY, square::EMPTY, square::EMPTY, 
                square::EMPTY, square::EMPTY, square::EMPTY, square::EMPTY,
            square::EMPTY, square::EMPTY, square::EMPTY, square::EMPTY, 
                square::EMPTY, square::EMPTY, square::EMPTY, square::EMPTY,
            color::WHITE, color::WHITE, color::WHITE, color::WHITE, 
                color::WHITE, color::WHITE, color::WHITE, color::WHITE,
            color::WHITE, color::WHITE, color::WHITE, color::WHITE, 
                color::WHITE, color::WHITE, color::WHITE, color::WHITE
        ];
        
        // iniciamos el vector historico con una jugada vacía y su contador tambien
        // cuando comparamos dos jugadas atrás capturas al paso
        let mut tmp_hist: Vec<mv::Shist> = Vec::new();
        let mut counter: usize = 0;
        
        for _ in 0..1 {
            let tmp = mv::Shist::new();
            tmp_hist.push(tmp);
            counter += 1;
        }

        //let tmp_castle = 15;      /* At start position all castle types ar available */

        // *********************************************
        // Now the calculation to create the Zobrist key from initial position
        // We need to flip the board 
        // *********************************************
        let mut tmp_hash: u64 = 0;
        
        //PIECES
        for i in 0..64 {
            if init_pos[i] != square::EMPTY {
                let row: usize = util::get_row(square::REVERSED[i]);
                let file: usize = util::get_col(square::REVERSED[i]);
                let poly_piece = pi::to_piece_poly(init_pos[i], init_colour[i]);

                assert!(poly_piece < pi::POLY_NON_PIECE);

                tmp_hash ^= zob::RANDOM_PIECE[64 * poly_piece + 8 * row + file];
            }
        }
        
        //CASTLE
        // as we have the four castles to our dispoaal...
        // 'K':
            tmp_hash ^= zob::RANDOM_CASTLE[0];
        // 'Q':
            tmp_hash ^= zob::RANDOM_CASTLE[1];
        // 'k':
            tmp_hash ^= zob::RANDOM_CASTLE[2];
        // 'q':
            tmp_hash ^= zob::RANDOM_CASTLE[3];

        //EN PASSANT
        //Nothing to do at start position

        //TURN
        // to_move == 'w'
            tmp_hash ^= zob::RANDOM_TURN[0];

        
        Sboard {
            piece: init_pos,
            color: init_colour,
            side: color::WHITE,
            hist: tmp_hist,
            hdp: counter, 
            count_make_move: 0,
            castle_rights: 15,      /* At start position all castle types ar available */
            ply: 1,
            ply_pawn: 0,
            en_passant: -1,
            hash_key: tmp_hash,
        }
    }

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


    fn reset_board(&mut self) {

        for index in 0..square::BRD_SQ_NUM {
            self.piece[index] = square::EMPTY;
        }
        
        for index in 0..square::BRD_SQ_NUM {
            self.color[index] = square::EMPTY;
        }
        
        let mut tmp_hist: Vec<mv::Shist> = Vec::new();
        let mut counter: usize = 0;
        
        for _ in 0..1 {
            let tmp = mv::Shist::new();
            tmp_hist.push(tmp);
            counter += 1;
        }

        self.side = color::WHITE;
        self.hist = tmp_hist;
        self.hdp = counter;
        self.count_make_move = 0;
        self.castle_rights = 0;      /* At start position all castle types ar available */
        self.ply = 1;
        self.ply_pawn = 0;
        self.en_passant = -1;
        self.hash_key = 0;
    }


    pub fn set_fen(&mut self, fen: &str) -> bool {
	
        assert!(!fen.is_empty());
        
        let mut rank: i16 = util::RANK_MIN as i16;
        let mut file: i16 = util::FILE_MIN as i16;
        let mut piece: usize;
        let mut color: usize;
        let mut count: usize;
        let mut j = 0usize; 
        let mut v_sq64: usize;
        let mut curr_char: &str;    // current char into the fen
        let trim_fen = fen.trim();
    
        self.reset_board();
    
        while (rank <= util::RANK_MAX as i16) && j < fen.len() {
            count = 1;
            curr_char = &trim_fen[j..j+1];

            match curr_char {
                "p" => { piece = pi::PAWN;   color = color::BLACK; },
                "r" => { piece = pi::ROOK;   color = color::BLACK; },
                "n" => { piece = pi::KNIGHT; color = color::BLACK; },
                "b" => { piece = pi::BISHOP; color = color::BLACK; },
                "k" => { piece = pi::KING;   color = color::BLACK; },
                "q" => { piece = pi::QUEEN;  color = color::BLACK; },
                "P" => { piece = pi::PAWN;   color = color::WHITE; },
                "R" => { piece = pi::ROOK;   color = color::WHITE; },
                "N" => { piece = pi::KNIGHT; color = color::WHITE; },
                "B" => { piece = pi::BISHOP; color = color::WHITE; },
                "K" => { piece = pi::KING;   color = color::WHITE; },
                "Q" => { piece = pi::QUEEN;  color = color::WHITE; },
                // ahora ASCII : "0" = 48, "1" = 49,..., "8" = 56
                "1" => { piece = square::EMPTY; color = square::EMPTY; count = 49 - 48; },
                "2" => { piece = square::EMPTY; color = square::EMPTY; count = 50 - 48; },
                "3" => { piece = square::EMPTY; color = square::EMPTY; count = 51 - 48; },
                "4" => { piece = square::EMPTY; color = square::EMPTY; count = 52 - 48; },
                "5" => { piece = square::EMPTY; color = square::EMPTY; count = 53 - 48; }, 
                "6" => { piece = square::EMPTY; color = square::EMPTY; count = 54 - 48; },
                "7" => { piece = square::EMPTY; color = square::EMPTY; count = 55 - 48; },
                "8" => { piece = square::EMPTY; color = square::EMPTY; count = 56 - 48; },

                "/" => { rank += 1; file = util::FILE_MIN as i16; j += 1; continue; },
                " " => { rank += 1; file = util::FILE_MIN as i16; j += 1; continue; },
    
                _ => { print!("FEN error \n"); return false; },
            };
            
            for _ in 0..count {
                v_sq64 = (rank * 8 + file) as usize;
                self.piece[v_sq64] = piece;
                self.color[v_sq64] = color;
                
                file += 1;
            }
            j += 1;
        }

        // *********************************************
        // Now the calculation to create the Zobrist key from initial position
        // We need to flip the board 
        // *********************************************

        let mut tmp_hash: u64 = 0;
        
        // HASH PIECES
        for i in 0..64 {
            if self.piece[i] != square::EMPTY && 
            self.piece[i] != square::EPS_SQUARE {
                let row: usize = util::get_row(square::REVERSED[i]);
                let file: usize = util::get_col(square::REVERSED[i]);
                let poly_piece = pi::to_piece_poly(self.piece[i], self.color[i]);

                assert!(poly_piece < pi::POLY_NON_PIECE);

                tmp_hash ^= zob::RANDOM_PIECE[64 * poly_piece + 8 * row + file];
            }
        }


        // side to move
        if &trim_fen[j..j+1] == "w" {
            self.side = color::WHITE;
            tmp_hash ^= zob::RANDOM_TURN[0];
        }
        else if &trim_fen[j..j+1] == "b" {
            self.side = color::BLACK;
        }
        else {
            eprintln!("error in side to move {}", &trim_fen[j..j+1]);
            return false;
        }
        j += 2;     // 2 is by the next space

        // now the castles
        for _ in 0..4 {
            if &trim_fen[j..j+1] == " " {
                break;
            }		
            match &trim_fen[j..j+1] {
                "K" => {
                    self.castle_rights |= mv::Enroques::WKCA as usize;
                    tmp_hash ^= zob::RANDOM_CASTLE[0];
                },
                "Q" => {
                    self.castle_rights |= mv::Enroques::WQCA as usize;
                    tmp_hash ^= zob::RANDOM_CASTLE[1];
                },
                "k" => {
                    self.castle_rights |= mv::Enroques::BKCA as usize;
                    tmp_hash ^= zob::RANDOM_CASTLE[2];
                },
                "q" => {
                    self.castle_rights |= mv::Enroques::BQCA as usize;
                    tmp_hash ^= zob::RANDOM_CASTLE[3];
                },
                _ => (),
            };
            j += 1;
        }
        j += 1;

        assert!(self.castle_rights <= 15);
        
        // en passant capture
        if &trim_fen[j..j+1] != "-" {
            curr_char = &trim_fen[j..j+2];
            
            let opt = square::ALGEBRA_SQUARE.iter().position(|&r| r == curr_char);    //.unwrap();
            match opt {
                Some(idx) => {
                    // the range of valid en passant squares a6-h6 and a3-h3
                    let col = util::get_col(idx);
                    match idx {
                        16..=23 => {
                            self.en_passant = idx as i16;
                            self.piece[idx] = square::EPS_SQUARE;
                            tmp_hash ^= zob::RANDOM_EN_PASSANT[col];
                        },
                        40..=47 => {
                            self.en_passant = idx as i16;
                            self.piece[idx] = square::EPS_SQUARE;
                            tmp_hash ^= zob::RANDOM_EN_PASSANT[col];
                        },
                        _ => {
                            eprintln!("no valid range in ep-square : {}", curr_char);
                            return false;
                        }

                    };
                },
                None => {
                    eprintln!("no valid ep-square : {}", curr_char);
                    return false;
                },
            };
            j += 2;
        }
        else { j+=1; }

        j += 1; // skip the space

        let split = trim_fen[j..].split(" ");
        let plies: Vec<&str> = split.collect();
        if plies.len() != 2 {return false;}

        self.ply_pawn = plies[0].parse::<usize>().unwrap();
        self.ply = plies[1].parse::<usize>().unwrap();

        self.hash_key = tmp_hash;
let mut clon = self.clone();
assert_eq!(self.hash_key, zob::hash_key(&mut clon));
        true
    }


    /**
    *  Returns the current position in FEN-notation
    *
    *  return A string with FEN-notation
    */
    pub fn get_fen(&mut self) -> String {
        let mut fen_string = "".to_string(); // This holds the FEN-string

        // ***
        // The following lines adds the pieces and empty squares to the FEN
        // ***

        let mut index: usize = 0;     // Keeps track of the index on the board
        let mut empties: i32 = 0;       // Number of empty squares in a row
        let mut char_piece: char = ' ';
        
        while index < 64 {
            if self.piece[index] != square::EMPTY &&  // If a piece is on the square
            self.piece[index] != square::EPS_SQUARE
                // i.e. the square it not empty
            {
                if empties != 0 {
                    // Add the empty square number
                    fen_string = format!("{}{}", fen_string, empties); 
                    // if it's not 0
                }
                empties = 0; // Reset empties (since we now have a piece coming)
            }

            match self.piece[index] {
                // Add the piece on the square
                pi::KING   => {
                    if self.color[index] == color::WHITE {
                        char_piece = 'K';
                    }
                    else if self.color[index] == color::BLACK {
                        char_piece = 'k';
                    }
                    fen_string.push(char_piece);
                },
                pi::QUEEN  => {
                    if self.color[index] == color::WHITE {
                        char_piece = 'Q';
                    }
                    else if self.color[index] == color::BLACK {
                        char_piece = 'q';
                    }
                    fen_string.push(char_piece);
                },
                pi::ROOK   => {
                    if self.color[index] == color::WHITE {
                        char_piece = 'R';
                    }
                    else if self.color[index] == color::BLACK {
                        char_piece = 'r';
                    }
                    fen_string.push(char_piece);
                },
                pi::BISHOP => {
                    if self.color[index] == color::WHITE {
                        char_piece = 'B';
                    }
                    else if self.color[index] == color::BLACK {
                        char_piece = 'b';
                    }
                    fen_string.push(char_piece);
                },
                pi::KNIGHT => {
                    if self.color[index] == color::WHITE {
                        char_piece = 'N';
                    }
                    else if self.color[index] == color::BLACK {
                        char_piece = 'n';
                    }
                    fen_string.push(char_piece);
                },
                pi::PAWN   => {
                    if self.color[index] == color::WHITE {
                        char_piece = 'P';
                    }
                    else if self.color[index] == color::BLACK {
                        char_piece = 'p';
                    }
                    fen_string.push(char_piece);
                },
                
                _ => empties += 1, // If no piece, increment the empty square count
            };
            index += 1; // Go to the next square

            if index % 8 == 0 {   // Reached the end of a rank
                if empties != 0 {
                    // Add the empties number if it's not 0
                    fen_string = format!("{}{}", fen_string, empties);  // Add the empties number if it's not 0
                    empties = 0;
                }
                if index < 63 {
                    fen_string += "/";   // Add to mark a new rank, if we're not at the end
                }
            }
            
        }
        // END Adding pieces

        fen_string = format!("{}{}", fen_string, " "); // Add space for next part
        // Adds side to move (important space before the letter here)
        if self.side == color::WHITE {
            fen_string = format!("{}{}", fen_string, "w");   // White's move
        }
        else {
            fen_string = format!("{}{}", fen_string, "b");  //Black's move
        }

        fen_string = format!("{}{}", fen_string, " "); // Add space for next part
        // Castling rights
        if self.castle_rights == 0 {
            fen_string.push('-');
        }
        else {
            // da 0 si el bit correspondiente esta a cero
            // white short
            let wk_side = self.castle_rights & mv::Enroques::WKCA as usize;
            if wk_side != 0 { fen_string.push('K'); }
            let wq_side = self.castle_rights & mv::Enroques::WQCA as usize;
            if wq_side != 0 { fen_string.push('Q'); }
            let bk_side = self.castle_rights & mv::Enroques::BKCA as usize;
            if bk_side != 0 { fen_string.push('k'); }
            let bq_side = self.castle_rights & mv::Enroques::BQCA as usize;
            if bq_side != 0 { fen_string.push('q'); }
        }

        fen_string = format!("{}{}", fen_string, " "); // Add space for next part

        // En passant square
		if self.en_passant == -1 {
            fen_string = format!("{}{}", fen_string, "-"); // If no en passant is available
        }
        else {
            let idx_square = square::ALGEBRA_SQUARE[self.en_passant as usize];
            fen_string = format!("{}{}", fen_string, idx_square);
        }
        
        fen_string = format!("{}{}", fen_string, " "); // Add space for next part
		fen_string = format!("{}{}", fen_string, self.ply_pawn); // Add half-moves since last capture/pawn move
		fen_string = format!("{}{}", fen_string, " ");
		fen_string = format!("{}{}", fen_string, self.ply); // Add number of full moves in the game so far

        fen_string
    }
    // END get_fen()


    // ===================================
    // Helper methods
    // ===================================

    pub fn get_board_flags (&mut self) -> usize {
        let mut flags: usize = 0;
    
        // comprobar si esta disponible enroque corto del blanco
        if self.piece[square::H1] == pi::ROOK &&
                self.piece[square::E1] == pi::KING {
            flags |= 1 << 0;
        }
        // comprobar si esta disponible enroque largo del blanco
        if self.piece[square::A1] == pi::ROOK &&
                self.piece[square::E1] == pi::KING {
            flags |= 1 << 1;
        }
        // comprobar si esta disponible enroque corto del negro
        if self.piece[square::H8] == pi::ROOK &&
                self.piece[square::E8] == pi::KING {
            flags |= 1 << 2;
        }
        // comprobar si esta disponible enroque largo del negro
        if self.piece[square::A8] == pi::ROOK &&
                self.piece[square::E8] == pi::KING {
            flags |= 1 << 3;
        }
    
        flags
    }



    pub fn print_board(self) {

        //let mut fen: String = String::from("");
        let piece_name: [char; 12] = ['P','N','B','R','Q','K',
                                    'p','n','b','r','q','k'];
        
        for i in 0..64 {
            if (i & 7) == 0 {
                print! ("   |---|---|---|---|---|---|---|---|\n");
                if i <= 56 {
                    print! (" {} |", 8 - (i >> 3));
                }
            }
            if self.piece[i] == square::EMPTY && ((i >> 3) % 2 == 0 && i % 2 == 0) {
                print! ("   |");
            }
            else if self.piece[i] == square::EMPTY && (( i >> 3) % 2 != 0 && i % 2 != 0) {
                print! ("   |");
            }
            else if self.piece[i] == square::EMPTY {
                print! ("   |");
            }
            else if self.piece[i] == square::EPS_SQUARE {
                print! (" * |");
            }
            else {
                if self.color[i] == color::WHITE {
                    print! (" {} |", piece_name[self.piece[i]]);
                }
                else {
                    print! (" {} |", piece_name[self.piece[i] + 6]);
                }
            }
            
            if (i & 7) == 7 {
                print! ("\n");
            }
        }
        print! ("   |---|---|---|---|---|---|---|---|\n     a   b   c   d   e   f   g   h\n");
    }


    pub fn get_side(&mut self) -> usize {
        return self.side;
    }



    // ============================================
    // relative to generation of legal moves ...
    // ============================================

    /* Gen all moves of current_side to move and push them to pBuf, 
    and return number of moves */

    pub fn gen_moves (&mut self, mut p_buf: &mut Vec<mv::Smove>) -> i32 {
        // i;			/* Counter for the board squares */
        let mut k: i16;			/* Counter for cols */
        let mut y: i16;
        let mut row: usize;
        let mut col: usize;
        let mut movecount: i32 = 0;

        let current_side = self.get_side();

        for i in 0..64 {	/* Scan all board */
            if self.color[i] == current_side {
                
                match self.piece[i] {

                    pi::PAWN => {
                        col = util::get_col (i);
                        row = util::get_row (i);
                    
                        if current_side == color::BLACK {
                            if self.color[i + 8] == square::EMPTY {
                                /* Pawn advances one square.
                                * We use Gen_PushPawn because it can be a promotion */
                                self.gen_push_pawn (i, i + 8, &mut p_buf, &mut movecount);
                            }
                            if row == 1 && self.color[i + 8] == square::EMPTY
                                    && self.color[i + 16] == square::EMPTY {
                                /* Pawn advances two squares */
                                self.gen_push_pawn_two (i, i + 16, &mut p_buf, &mut movecount);
                            }
                            if col != 0 && self.color[i + 7] == color::WHITE {
                                /* Pawn captures and it can be a promotion */
                                self.gen_push_pawn (i, i + 7, &mut p_buf, &mut movecount);
                            }
                            if col < 7 && self.color[i + 9] == color::WHITE {
                                /* Pawn captures and can be a promotion */
                                self.gen_push_pawn (i, i + 9, &mut p_buf, &mut movecount);
                            }
                            /* For en passant capture */
                            if col != 0 && self.piece[i + 7] == square::EPS_SQUARE {
                                /* Pawn captures and it can be a promotion */
                                self.gen_push_pawn (i, i + 7, &mut p_buf, &mut movecount);
                            }
                            if col < 7 && self.piece[i + 9] == square::EPS_SQUARE {
                                /* Pawn captures and can be a promotion */
                                self.gen_push_pawn (i, i + 9, &mut p_buf, &mut movecount);
                            }
                        }
                        else {
                            if self.color[i - 8] == square::EMPTY {
                                self.gen_push_pawn (i, i - 8, &mut p_buf, &mut movecount);
                            }
                            /* Pawn moves 2 squares */
                            if row == 6 && self.color[i - 8] == square::EMPTY
                                    && self.color[i - 16] == square::EMPTY {
                                self.gen_push_pawn_two (i, i - 16, &mut p_buf, &mut movecount);
                            }
                            /* For captures */
                            if col != 0 && self.color[i - 9] == color::BLACK {
                                self.gen_push_pawn (i, i - 9, &mut p_buf, &mut movecount);
                            }
                            if col < 7 && self.color[i - 7] == color::BLACK {
                                self.gen_push_pawn (i, i - 7, &mut p_buf, &mut movecount);
                            }
                            /* For en passant capture */
                            if col != 0 && self.piece[i - 9] == square::EPS_SQUARE {
                                self.gen_push_pawn (i, i - 9, &mut p_buf, &mut movecount);
                            }
                            if col < 7 && self.piece[i - 7] == square::EPS_SQUARE {
                                self.gen_push_pawn (i, i - 7, &mut p_buf, &mut movecount);
                            }
                        }
                    },

                    pi::QUEEN => {     /* == BISHOP+ROOK */
                        // BISHOP 

                        y = i as i16 -9;
                        while y >= 0 && util::get_col(y as usize) != 7 {
                            if self.color[y as usize] != current_side {
                                self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                            }
                            if self.color[y as usize] != square::EMPTY {
                                break;
                            }
                            y -= 9;
                        }
                        /* go right up */
                        y = i as i16 -7;
                        while y >= 0 && util::get_col(y as usize) != 0 {
                            if self.color[y as usize] != current_side {
                                self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                            }
                            if self.color[y as usize] != square::EMPTY {
                                break;
                            }
                            y -= 7;
                        }
                        /* go right down */
                        y = i as i16 + 9;
                        while y < 64 && util::get_col(y as usize) != 0 {
                            if self.color[y as usize] != current_side {
                                self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                            }
                            if self.color[y as usize] != square::EMPTY {
                                break;
                            }
                            y += 9;
                        }
                        /* go left down */
                        y = i as i16 + 7;
                        while y < 64 && util::get_col(y as usize) != 7 {  // row/col never negative -> usize
                            if self.color[y as usize] != current_side {
                                self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                            }
                            if self.color[y as usize] != square::EMPTY {
                                break;
                            }
                            y += 7;
                        }

                        // ROOK

                        col = util::get_col(i);
                        /* go left */
                        k = (i - col) as i16;
                        y = i as i16 - 1;
                        while y >= k {
                            if self.color[y as usize] != current_side {
                                self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                            }
                            if self.color[y as usize] != square::EMPTY {
                                break;
                            }
                            y -= 1;
                        }
                        /* go right */
                        k = (i - col) as i16 + 7;
                        y = i as i16 + 1;
                        while y <= k {
                            if self.color[y as usize] != current_side {
                                self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                            }
                            if self.color[y as usize] != square::EMPTY {
                                break;
                            }
                            y += 1;
                        }
                        /* go up */
                        y = i as i16 - 8;
                        while y >= 0 {
                            if self.color[y as usize] != current_side {
                                self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                            }
                            if self.color[y as usize] != square::EMPTY {
                                break;
                            }
                            y -= 8;
                        }
                        /* go down */
                        y = i as i16 + 8;
                        while y < 64 {
                            if self.color[y as usize] != current_side {
                                self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                            }
                            if self.color[y as usize] != square::EMPTY {
                                break;
                            }
                            y += 8;
                        }
                    }		
                    pi::BISHOP => {
                        /* go left up */
                        y = i as i16 -9;
                        while y >= 0 && util::get_col(y as usize) != 7 {
                            if self.color[y as usize] != current_side {
                                self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                            }
                            if self.color[y as usize] != square::EMPTY {
                                break;
                            }
                            y -= 9;
                        }
                        /* go right up */
                        y = i as i16 -7;
                        while y >= 0 && util::get_col(y as usize) != 0 {
                            if self.color[y as usize] != current_side {
                                self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                            }
                            if self.color[y as usize] != square::EMPTY {
                                break;
                            }
                            y -= 7;
                        }
                        /* go right down */
                        y = i as i16 + 9;
                        while y < 64 && util::get_col(y as usize) != 0 {
                            if self.color[y as usize] != current_side {
                                self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                            }
                            if self.color[y as usize] != square::EMPTY {
                                break;
                            }
                            y += 9;
                        }
                        /* go left down */
                        y = i as i16 + 7;
                        while y < 64 && util::get_col(y as usize) != 7 {  // row/col never negative -> usize
                            if self.color[y as usize] != current_side {
                                self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                            }
                            if self.color[y as usize] != square::EMPTY {
                                break;
                            }
                            y += 7;
                        }
                        if self.piece[i] == pi::BISHOP {	/* In the case of the bishop we're done */
                            //break;
                        }
                    },
                    
                    pi::ROOK => {
                        col = util::get_col(i);
                        /* go left */
                        k = (i - col) as i16;
                        y = i as i16 - 1;
                        while y >= k {
                            if self.color[y as usize] != current_side {
                                self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                            }
                            if self.color[y as usize] != square::EMPTY {
                                break;
                            }
                            y -= 1;
                        }
                        /* go right */
                        k = (i - col) as i16 + 7;
                        y = i as i16 + 1;
                        while y <= k {
                            if self.color[y as usize] != current_side {
                                self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                            }
                            if self.color[y as usize] != square::EMPTY {
                                break;
                            }
                            y += 1;
                        }
                        /* go up */
                        y = i as i16 - 8;
                        while y >= 0 {
                            if self.color[y as usize] != current_side {
                                self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                            }
                            if self.color[y as usize] != square::EMPTY {
                                break;
                            }
                            y -= 8;
                        }
                        /* go down */
                        y = i as i16 + 8;
                        while y < 64 {
                            if self.color[y as usize] != current_side {
                                self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                            }
                            if self.color[y as usize] != square::EMPTY {
                                break;
                            }
                            y += 8;
                        }
                    },

                    pi::KNIGHT => {
                        col = util::get_col(i);
                        y = i as i16 - 6;
                        
                        if y >= 0 && col < 6 && self.color[y as usize] != current_side {
                            self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                        }
                        y = i as i16 - 10;
                        if y >= 0 && col > 1 && self.color[y as usize] != current_side {
                            self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                        }
                        y = i as i16 - 15;
                        if y >= 0 && col < 7 && self.color[y as usize] != current_side {
                            self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                        }
                        y = i as i16 - 17;
                        if y >= 0 && col > 0 && self.color[y as usize] != current_side {
                            self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                        }
                        y = i as i16 + 6;
                        if y < 64 && col > 1 && self.color[y as usize] != current_side {
                            self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                        }
                        y = i as i16 + 10;
                        if y < 64 && col < 6 && self.color[y as usize] != current_side {
                            self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                        }
                        y = i as i16 + 15;
                        if y < 64 && col > 0 && self.color[y as usize] != current_side {
                            self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                        }
                        y = i as i16 + 17;
                        if y < 64 && col < 7 && self.color[y as usize] != current_side {
                            self.gen_push_normal (i, y as usize, &mut p_buf, &mut movecount);
                        }
                    },

                    pi::KING => {
                        /* the column and rank checks are to make sure it is on the board */
                        /* The 'normal' moves */
                        col = util::get_col(i);
                        if col != 0 && self.color[i - 1] != current_side {
                            self.gen_push_king (i, i - 1, &mut p_buf, &mut movecount);	/* left */
                        }
                        if col < 7 && self.color[i + 1] != current_side {
                            self.gen_push_king (i, i + 1, &mut p_buf, &mut movecount);	/* right */
                        }
                        if i > 7 && self.color[i - 8] != current_side {
                            self.gen_push_king (i, i - 8, &mut p_buf, &mut movecount);	/* up */
                        }
                        if i < 56 && self.color[i + 8] != current_side {
                            self.gen_push_king (i, i + 8, & mut p_buf, &mut movecount);	/* down */
                        }
                        if col !=0 && i > 7 && self.color[i - 9] != current_side {
                            self.gen_push_king (i, i - 9, &mut p_buf, &mut movecount);	/* left up */
                        }
                        if col < 7 && i > 7 && self.color[i - 7] != current_side {
                            self.gen_push_king (i, i - 7, &mut p_buf, &mut movecount);	/* right up */
                        }
                        if col != 0 && i < 56 && self.color[i + 7] != current_side {
                            self.gen_push_king (i, i + 7, &mut p_buf, &mut movecount);	/* left down */
                        }
                        if col < 7 && i < 56 && self.color[i + 9] != current_side {
                            self.gen_push_king (i, i + 9, &mut p_buf, &mut movecount);	/* right down */
                        }
                        /* The castle moves */
                        if current_side == color::WHITE {
                            /* Can white short castle? */
                            if (self.castle_rights & 1) != 0 {
                                /* If white can castle the white king has to be in square 60 */
                                if col != 0 &&
                                self.color[i + 1] == square::EMPTY &&
                                self.color[i + 2] == square::EMPTY &&
                                !self.is_in_check (current_side) &&
                                !self.is_attacked (current_side, i as i16 + 1) {
                                    /* The king goes 2 sq to the left */
                                    self.gen_push_king (i, i + 2, &mut p_buf, &mut movecount);
                                }
                            }

                            /* Can white long castle? */
                            if (self.castle_rights & 2) != 0 {
                                if col != 0 &&
                                self.color[i - 1] == square::EMPTY &&
                                self.color[i - 2] == square::EMPTY &&
                                self.color[i - 3] == square::EMPTY &&
                                !self.is_in_check (current_side) &&
                                !self.is_attacked (current_side, i as i16 - 1) {
                                    /* The king goes 2 sq to the left */
                                    self.gen_push_king (i, i - 2, &mut p_buf, &mut movecount);
                                }
                            }
                        }
                        else if current_side == color::BLACK {
                            /* Can black short castle? */
                            if (self.castle_rights & 4) != 0 {
                                /* If white can castle the white king has to be in square 60 */
                                if col != 0 &&
                                self.color[i + 1] == square::EMPTY &&
                                self.color[i + 2] == square::EMPTY &&
                                self.piece[i + 3] == pi::ROOK &&
                                !self.is_in_check (current_side) &&
                                !self.is_attacked (current_side, i as i16 + 1) {
                                    /* The king goes 2 sq to the left */
                                    self.gen_push_king (i, i + 2, &mut p_buf, &mut movecount);
                                }
                            }
                            /* Can black long castle? */
                            if (self.castle_rights & 8) != 0 {
                                if col != 0 &&
                                self.color[i - 1] == square::EMPTY &&
                                self.color[i - 2] == square::EMPTY &&
                                self.color[i - 3] == square::EMPTY &&
                                self.piece[i - 4] == pi::ROOK &&
                                !self.is_in_check (current_side) &&
                                !self.is_attacked (current_side, i as i16 - 1) {
                                    /* The king goes 2 sq to the left */
                                    self.gen_push_king (i, i - 2, &mut p_buf, &mut movecount);
                                }
                            }
                        }
                    },
                    _ => (),
                                        //                default:
                                        //                printf("Piece type unknown, %d", piece[i]);
                                        // assert(false);
                    }
                }
        }
        return movecount;
    }


    /* Especial cases for Pawn */

    /* Pawn can promote */
    fn gen_push_pawn (&mut self, from: usize, dest: usize, 
    mut p_buf: &mut Vec<mv::Smove>, mut p_mcount: &mut i32) {
        /* The 7 and 56 are to limit pawns to the 2nd through 7th ranks, which
        * means this isn't a promotion, i.e., a normal pawn move */
        if self.piece[dest] == square::EPS_SQUARE {
            self.gen_push (from, dest, mv::MOVE_TYPE_EPS, &mut p_buf, &mut p_mcount);
        }
        else if dest > 7 && dest < 56	{       /* this is just a normal move */
            self.gen_push (from, dest, mv::MOVE_TYPE_NORMAL, &mut p_buf, &mut p_mcount);
        }
        else {				/* otherwise it's a promotion */
            self.gen_push (from, dest, mv::MOVE_TYPE_PROMOTION_TO_QUEEN, &mut p_buf, &mut p_mcount);
            self.gen_push (from, dest, mv::MOVE_TYPE_PROMOTION_TO_ROOK, &mut p_buf, &mut p_mcount);
            self.gen_push (from, dest, mv::MOVE_TYPE_PROMOTION_TO_BISHOP, &mut p_buf, &mut p_mcount);
            self.gen_push (from, dest, mv::MOVE_TYPE_PROMOTION_TO_KNIGHT, &mut p_buf, &mut p_mcount);
        }
    }

    /* When a pawn moves two squares then appears the possibility of the en passanta capture*/
    fn gen_push_pawn_two (&mut self, from: usize, dest: usize, 
    mut p_buf: &mut Vec<mv::Smove>, mut p_mcount: &mut i32) {
        self.gen_push (from, dest, mv::MOVE_TYPE_PAWN_TWO, &mut p_buf, &mut p_mcount);
    }


    fn gen_push_normal (&mut self, from: usize, dest: usize, 
    p_buf: &mut Vec<mv::Smove>, p_mcount: &mut i32) {
        self.gen_push (from, dest, mv::MOVE_TYPE_NORMAL, p_buf, p_mcount);
    }


    fn gen_push (&mut self, from: usize, dest: usize, tipe: usize, 
                p_buf: &mut Vec<mv::Smove>, p_mcount: &mut i32) {
        let mut mov : mv::Smove = mv::Smove::new();
        mov.from = from;
        mov.dest = dest;
        mov.tipe = tipe;
        
        if tipe >= mv::MOVE_TYPE_PROMOTION_TO_QUEEN {
            match tipe {
                mv::MOVE_TYPE_PROMOTION_TO_QUEEN => 
                    mov.promotion_poly = mv::PROMOTION_POLY_QUEEN,
                mv::MOVE_TYPE_PROMOTION_TO_ROOK =>
                    mov.promotion_poly = mv::PROMOTION_POLY_ROOK,
                mv::MOVE_TYPE_PROMOTION_TO_BISHOP =>
                    mov.promotion_poly = mv::PROMOTION_POLY_BISHOP,
                mv::MOVE_TYPE_PROMOTION_TO_KNIGHT =>
                    mov.promotion_poly = mv::PROMOTION_POLY_KNIGHT,
                _ => mov.promotion_poly = mv::PROMOTION_POLY_NONE,
            }
        }
        else {
            mov.promotion_poly = mv::PROMOTION_POLY_NONE;
        }

        /*
        mov.pos_before_move.square_pieces = self.piece.clone();
        mov.pos_before_move.square_colors = self.color;
        mov.pos_before_move.side = self.side;
        mov.pos_before_move.castles = self.castle_rights;
        mov.pos_before_move.passant = self.en_passant;
        mov.pos_before_move.hash = self.hash_key;
        */
        mov.encode_move();
        mov.hash = self.hash_key;
        //pBuf[*pMCount] = move;
        p_buf.push(mov);
        
        *p_mcount = *p_mcount + 1;
    }


    /* Especial cases for King */
    fn gen_push_king (&mut self, from: usize, dest: usize, 
    mut p_buf: &mut Vec<mv::Smove>, mut p_mcount: &mut i32) {
        /* Is it a castle? */
        if from == square::E1 && (dest == square::G1 || dest == square::C1) {	/* this is a white castle */
            self.gen_push (from, dest, mv::MOVE_TYPE_CASTLE, &mut p_buf, &mut p_mcount);
        }
        else if from == square::E8 && (dest == square::G8 || dest == square::C8) {	/* this is a black castle */
            self.gen_push (from, dest, mv::MOVE_TYPE_CASTLE, &mut p_buf, &mut p_mcount);
        }
        else {				/* otherwise it's a normal king's move */
            self.gen_push (from, dest, mv::MOVE_TYPE_NORMAL, &mut p_buf, &mut p_mcount);
        }
    }


    /* Check if current side is in check. Necesary in order to check legality of moves
    and check if castle is allowed */

    fn is_in_check (&mut self, current_side: usize) -> bool {
        let mut k1: i16 = 0;			/* The square where the king is placed */

        /* Find the King of the side to move */
        for k in 0..64 {
            k1 = k as i16;
            if self.piece[k] == pi::KING && self.color[k] == current_side {
                break;
            }
        }

        /* Use IsAttacked in order to know if current_side is under check */
        let res = self.is_attacked (current_side, k1);
        return res;
    }

    /* Returns 1/true if square k is attacked by current_side, 0/false otherwise. Necesary, v.g., to check
    * castle rules (if king goes from e1 to g1, f1 can't be attacked by an enemy piece) */
    fn is_attacked (&mut self, current_side: usize, k: i16) -> bool {
        let mut h: i16;
        let mut y: i16;

        let xside: usize = (color::WHITE + color::BLACK) - current_side;	/* opposite current_side, who may be attacking */

        /* Situation of the square */
        let row: usize = util::get_row(k as usize);
        let col: usize = util::get_col(k as usize);

        /* Check Knight attack */
        if col > 0 && row > 1 && self.color[(k - 17) as usize] == xside && 
                self.piece[(k - 17) as usize] == pi::KNIGHT {
            return true;
        }
        if col < 7 && row > 1 && self.color[(k - 15) as usize] == xside && 
                self.piece[(k - 15) as usize] == pi::KNIGHT {
            return true;
        }
        if col > 1 && row > 0 && self.color[(k - 10) as usize] == xside && 
                self.piece[(k - 10) as usize] == pi::KNIGHT {
            return true;
        }
        if col < 6 && row > 0 && self.color[(k - 6) as usize] == xside && 
                self.piece[(k - 6) as usize] == pi::KNIGHT {
            return true;
        }
        if col > 1 && row < 7 && self.color[(k + 6) as usize] == xside && 
                self.piece[(k + 6) as usize] == pi::KNIGHT {
            return true;
        }
        if col < 6 && row < 7 && self.color[(k + 10) as usize] == xside && 
                self.piece[(k + 10) as usize] == pi::KNIGHT {
            return true;
        }
        if col > 0 && row < 6 && self.color[(k + 15) as usize] == xside && 
                self.piece[(k + 15) as usize] == pi::KNIGHT {
            return true;
        }
        if col < 7 && row < 6 && self.color[(k + 17) as usize] == xside && 
                self.piece[(k + 17) as usize] == pi::KNIGHT {
            return true;
        }

        /* Check horizontal and vertical lines for attacking of Queen, Rook, King */
        /* go down */
        y = k + 8;
        if y < 64 {
            if self.color[y as usize] == xside && (
                    self.piece[y as usize] == pi::KING || 
                    self.piece[y as usize] == pi::QUEEN || 
                    self.piece[y as usize] == pi::ROOK ) {
                return true;
            }
            if self.piece[y as usize] == square::EMPTY || 
                    self.piece[y as usize] == square::EPS_SQUARE {
                
                y += 8;
                while y < 64 {
                    if self.color[y as usize] == xside && (
                            self.piece[y as usize] == pi::QUEEN || 
                            self.piece[y as usize] == pi::ROOK ){
                        return true;
                    }
                    if self.piece[y as usize] != square::EMPTY && 
                            self.piece[y as usize] != square::EPS_SQUARE {
                        break;
                    }
                    y += 8;
                }
            }
        }

        /* go left */
        y = k - 1;
        h = k - col as i16;
        if y >= h {
            if self.color[y as usize] == xside && (
                    self.piece[y as usize] == pi::KING || 
                    self.piece[y as usize] == pi::QUEEN || 
                    self.piece[y as usize] == pi::ROOK) {
                return true;
            }
            if self.piece[y as usize] == square::EMPTY || 
                    self.piece[y as usize] == square::EPS_SQUARE {
                
                y -= 1;
                while y >= h {
                    if self.color[y as usize] == xside && (
                            self.piece[y as usize] == pi::QUEEN ||
                            self.piece[y as usize] == pi::ROOK) {
                        return true;
                    }
                    if self.piece[y as usize] != square::EMPTY && 
                            self.piece[y as usize] != square::EPS_SQUARE {
                        break;
                    }
                    y -= 1;
                }
            }
        }

        /* go right */
        y = k + 1;
        h = k - col as i16 + 7;
        if y <= h {
            if self.color[y as usize] == xside && (
                    self.piece[y as usize] == pi::KING || 
                    self.piece[y as usize] == pi::QUEEN || 
                    self.piece[y as usize] == pi::ROOK) {
                return true;
            }
            if self.piece[y as usize] == square::EMPTY || 
                    self.piece[y as usize] == square::EPS_SQUARE {
                
                y += 1;
                while y <= h {
                    if self.color[y as usize] == xside && (
                            self.piece[y as usize] == pi::QUEEN ||
                            self.piece[y as usize] == pi::ROOK) {
                        return true;
                    }
                    if self.piece[y as usize] != square::EMPTY && 
                            self.piece[y as usize] != square::EPS_SQUARE {
                        break;
                    }
                    y += 1;
                }
            }
        }

        /* go up */
        y = k - 8;
        if y >= 0 {
            if self.color[y as usize] == xside && (
                    self.piece[y as usize] == pi::KING || 
                    self.piece[y as usize] == pi::QUEEN || 
                    self.piece[y as usize] == pi::ROOK) {
                return true;
            }
            if self.piece[y as usize] == square::EMPTY || 
                    self.piece[y as usize] == square::EPS_SQUARE {
                //for (y -= 8; y >= 0; y -= 8)
                y -= 8;
                while y >= 0 {
                    if self.color[y as usize] == xside && (
                            self.piece[y as usize] == pi::QUEEN || 
                            self.piece[y as usize] == pi::ROOK) {
                        return true;
                    }
                    if self.piece[y as usize] != square::EMPTY && 
                            self.piece[y as usize] != square::EPS_SQUARE {
                        break;
                    }
                    y -= 8;
                }
            }
        }

        /* Check diagonal lines for attacking of Queen, Bishop, King, Pawn */
        /* go right down */
        y = k + 9;
        if y < 64 && util::get_col(y as usize) != 0 {
            if self.color[y as usize] == xside {
                if self.piece[y as usize] == pi::KING || 
                        self.piece[y as usize] == pi::QUEEN || 
                        self.piece[y as usize] == pi::BISHOP {
                    return true;
                }
                if current_side == color::BLACK && 
                        self.piece[y as usize] == pi::PAWN {
                    return true;
                }
            }
            if self.piece[y as usize] == square::EMPTY || 
                    self.piece[y as usize] == square::EPS_SQUARE {
                
                y += 9;
                while y < 64 && util::get_col(y as usize) != 0 {
                    if self.color[y as usize] == xside && (
                            self.piece[y as usize] == pi::QUEEN || 
                            self.piece[y as usize] == pi::BISHOP) {
                        return true;
                    }
                    if self.piece[y as usize] != square::EMPTY && 
                            self.piece[y as usize] != square::EPS_SQUARE {
                        break;
                    }
                    y += 9;
                }
            }
        }

        /* go left down */
        y = k + 7;
        if y < 64 && util::get_col(y as usize) != 7 {
            if self.color[y as usize] == xside {
                if self.piece[y as usize] == pi::KING || 
                        self.piece[y as usize] == pi::QUEEN || 
                        self.piece[y as usize] == pi::BISHOP {
                    return true;
                }
                if current_side == color::BLACK && 
                        self.piece[y as usize] == pi::PAWN {
                    return true;
                }
            }

            if self.piece[y as usize] == square::EMPTY || 
                    self.piece[y as usize] == square::EPS_SQUARE {
                
                y += 7;
                while y < 64 && util::get_col(y as usize) != 7 {
                    if self.color[y as usize] == xside && (
                            self.piece[y as usize] == pi::QUEEN || 
                            self.piece[y as usize] == pi::BISHOP) {
                        return true;
                    }
                    if self.piece[y as usize] != square::EMPTY && 
                            self.piece[y as usize] != square::EPS_SQUARE {
                        break;
                    }
                    y += 7;
                }
            }
        }

        /* go left up */
        y = k - 9;
        if y >= 0 && util::get_col(y as usize) != 7 {
            if self.color[y as usize] == xside {
                if self.piece[y as usize] == pi::KING || 
                        self.piece[y as usize] == pi::QUEEN || 
                        self.piece[y as usize] == pi::BISHOP  {
                    return true;
                }
                if current_side == color::WHITE && 
                        self.piece[y as usize] == pi::PAWN {
                    return true;
                }
            }
            if self.piece[y as usize] == square::EMPTY || 
                    self.piece[y as usize] == square::EPS_SQUARE {
                
                y -= 9;
                while y >= 0 && util::get_col(y as usize) != 7 {
                    if self.color[y as usize] == xside && (
                            self.piece[y as usize] == pi::QUEEN || 
                            self.piece[y as usize] == pi::BISHOP) {
                        return true;
                    }
                    if self.piece[y as usize] != square::EMPTY && 
                            self.piece[y as usize] != square::EPS_SQUARE {
                        break;
                    }
                    y -= 9;
                }
            }
        }

        /* go right up */
        y = k - 7;
        if y >= 0 && util::get_col(y as usize) != 0 {
            if self.color[y as usize] == xside { 
                if self.piece[y as usize] == pi::KING || 
                        self.piece[y as usize] == pi::QUEEN || 
                        self.piece[y as usize] == pi::BISHOP {
                    return true;
                }
                if current_side == color::WHITE && 
                        self.piece[y as usize] == pi::PAWN {
                    return true;
                }
            }
            if self.piece[y as usize] == square::EMPTY || 
                    self.piece[y as usize] == square::EPS_SQUARE {
                //for (y -= 7; y >= 0 && COL (y) != 0; y -= 7)
                y -= 7;
                while y >= 0 && util::get_col(y as usize) != 0 {
                    if self.color[y as usize] == xside && (
                            self.piece[y as usize] == pi::QUEEN || 
                            self.piece[y as usize] == pi::BISHOP) {
                        return true;
                    }
                    if self.piece[y as usize] != square::EMPTY && 
                            self.piece[y as usize] != square::EPS_SQUARE {
                        break;
                    }
                    y -= 7;
                }
            }
        }

        return false;
    }


    // ============================================
    // relative to make and undo move ...
    // ============================================

    pub fn make_move (&mut self, m: &mut mv::Smove ) -> Option<mv::Smove> {

        //let mut tmp_hash: u64 = self.hash_key;

        /* We make room for this new move */
        let new_hist: mv::Shist = mv::Shist::new();
        self.hist.push(new_hist);

        /* increments the counter */ 
        self.count_make_move += 1;
        self.hdp = self.hist.len() - 1;
        
        let mut valid_move: Option<mv::Smove> = None;
        assert_eq!(self.hdp, self.hist.len()-1);

        self.hist[self.hdp].m = m.clone();
        self.hist[self.hdp].cap = self.piece[m.dest];	/* store in history the piece of the dest square */
        self.hist[self.hdp].castle = self.castle_rights;

        self.piece[m.dest] = self.piece[m.from];	/* dest piece is the one in the original square */
        self.color[m.dest] = self.color[m.from];	/* The dest square color is the one of the origin piece */
        

        /* ply pawn move */
        if self.piece[m.from] == pi::PAWN {
            self.ply_pawn = 0;
        }
        else {
            self.ply_pawn += 1;
        }

        self.piece[m.from] = square::EMPTY;	        /* The original square becomes empty */
        self.color[m.from] = square::EMPTY;	        /* The original color becomes empty */

        /* en pasant capture */
        if m.tipe == mv::MOVE_TYPE_EPS {
            if self.side == color::WHITE {
                self.piece[m.dest + 8] = square::EMPTY;
                self.color[m.dest + 8] = square::EMPTY;
            }
            else {
                self.piece[m.dest - 8] = square::EMPTY;
                self.color[m.dest - 8] = square::EMPTY;
            }
            self.en_passant = -1;
        }

        /* Remove possible eps piece, remaining from former move */
        //if self.hdp > 0 {
            if self.hist[self.hdp - 1].m.tipe == mv::MOVE_TYPE_PAWN_TWO {
                for i in 16..=23 {   //(i = 16; i <= 23; i++) {
                    if self.piece[i] == square::EPS_SQUARE {
                        self.piece[i] = square::EMPTY;
                        /* this seems unnecesary, but otherwise a bug occurs:
                        * after: a3 Nc6 d4 e6, white isn't allowed to play e4 */
                        //    color[i] = EMPTY;
                        //self.en_passant = -1;
                        break;
                    }
                }
                for i in 40..=47 {   //(i = 40; i <= 47; i++) {
                    if self.piece[i] == square::EPS_SQUARE {
                        self.piece[i] = square::EMPTY;
                        //   color[i] = EMPTY;
                        //self.en_passant = -1;
                        break;
                    }
                }
            }
        //}

        /* Add the eps square when a pawn moves two squares */
        if m.tipe == mv::MOVE_TYPE_PAWN_TWO {
            
            if self.side == color::BLACK {
                self.piece[m.from + 8] = square::EPS_SQUARE;
                self.color[m.from + 8] = square::EMPTY;
                self.en_passant = (m.from + 8) as i16;
            }
            else if self.side == color::WHITE {
                self.piece[m.from - 8] = square::EPS_SQUARE;
                self.color[m.from - 8] = square::EMPTY;
                self.en_passant = (m.from - 8) as i16;
            }
        }

        /* Once the move is done we check either this is a promotion */
        if m.tipe >= mv::MOVE_TYPE_PROMOTION_TO_QUEEN {
            /* In this case we put in the destiny sq the chosen piece */
            match m.tipe {
                mv::MOVE_TYPE_PROMOTION_TO_QUEEN => 
                        self.piece[m.dest] = pi::QUEEN,

                mv::MOVE_TYPE_PROMOTION_TO_ROOK => 
                        self.piece[m.dest] = pi::ROOK,
    
                mv::MOVE_TYPE_PROMOTION_TO_BISHOP => 
                        self.piece[m.dest] = pi::BISHOP,
    
                mv::MOVE_TYPE_PROMOTION_TO_KNIGHT => 
                        self.piece[m.dest] = pi::KNIGHT,
    
                _ => {
                    println! ("Impossible to get here...");
                    assert! (false);
                },
            }
        }

        if m.tipe == mv::MOVE_TYPE_CASTLE {
            if m.dest == square::G1 {
                /* h1-h8 becomes empty */
                self.piece[m.from + 3] = square::EMPTY;
                self.color[m.from + 3] = square::EMPTY;
                /* rook to f1-f8 */
                self.piece[m.from + 1] = pi::ROOK;
                self.color[m.from + 1] = color::WHITE;
            }
            else if m.dest == square::C1 {
                /* h1-h8 becomes empty */
                self.piece[m.from - 4] = square::EMPTY;
                self.color[m.from - 4] = square::EMPTY;
                /* rook to f1-f8 */
                self.piece[m.from - 1] = pi::ROOK;
                self.color[m.from - 1] = color::WHITE;
            }
            else if m.dest == square::G8 {
                /* h1-h8 becomes empty */
                self.piece[m.from + 3] = square::EMPTY;
                self.color[m.from + 3] = square::EMPTY;
                /* rook to f1-f8 */
                self.piece[m.from + 1] = pi::ROOK;
                self.color[m.from + 1] = color::BLACK;
            }
            else if m.dest == square::C8 {
                /* h1-h8 becomes empty */
                self.piece[m.from - 4] = square::EMPTY;
                self.color[m.from - 4] = square::EMPTY;
                /* rook to f1-f8 */
                self.piece[m.from - 1] = pi::ROOK;
                self.color[m.from - 1] = color::BLACK;
            }
        }

        /* Update ply and ply_pawn ans hdp */
        self.ply += 1;
        
        //self.hdp += 1;
        //self.hdp = self.hist.len() - 1;

        /* Update the castle rights */
        self.castle_rights &= mv::CASTLE_MASK[m.from] & mv::CASTLE_MASK[m.dest];        
    
        /* Checking if after making the move we're in check */
        let r = self.is_in_check (self.side);

        if !r {
            //let mut clon = self.clone();
            //self.hash_key = zob::hash_key(&mut clon);
            valid_move = Some(m.clone());
        }

        /* After making move, give turn to opponent */
        self.side = (color::WHITE + color::BLACK) - self.side;
        
        self.hash_key = zob::hash_key(&mut *self);

        return valid_move;
    }





    /* Undo what make_move did */
    /* This will not be used, only created for perft purposes 
    *  and debug the generation of moves
    */
    
    pub fn undo_move (&mut self, _m: &mut mv::Smove) {
    
        let side = (color::WHITE + color::BLACK) - self.side;
        self.side = side;
    
        //self.hdp -= 1;
        self.hdp = self.hist.len() -1;
        self.ply -= 1;

        self.piece[self.hist[self.hdp].m.from] = self.piece[self.hist[self.hdp].m.dest];
        self.piece[self.hist[self.hdp].m.dest] = self.hist[self.hdp].cap;
        self.color[self.hist[self.hdp].m.from] = self.side;

        /* Update castle rights */
        self.castle_rights = self.hist[self.hdp].castle;

        /* Return the captured material */
        if self.hist[self.hdp].cap != square::EMPTY && 
                self.hist[self.hdp].cap != square::EPS_SQUARE {
            self.color[self.hist[self.hdp].m.dest] = (color::WHITE + color::BLACK) - self.side;
        }
        else {
            self.color[self.hist[self.hdp].m.dest] = square::EMPTY;
        }

        /* Promotion */
        if self.hist[self.hdp].m.tipe >= mv::MOVE_TYPE_PROMOTION_TO_QUEEN {
            self.piece[self.hist[self.hdp].m.from] = pi::PAWN;
        }

        /* If pawn moved two squares in the former move, we have to restore
        * the eps square */

        //if self.hdp > 0 {
            if self.hist[self.hdp - 1].m.tipe == mv::MOVE_TYPE_PAWN_TWO {
                if side == color::WHITE {
                    self.piece[self.hist[self.hdp - 1].m.dest - 8] = square::EPS_SQUARE;
                    //self.en_passant = self.piece[self.hist[self.hdp - 1].m.dest - 8] as i16;
                }
                else if side == color::BLACK {
                    self.piece[self.hist[self.hdp - 1].m.dest + 8] = square::EPS_SQUARE;
                    //self.en_passant = self.piece[self.hist[self.hdp - 1].m.dest + 8] as i16;
                }
            }
        //}

        /* To remove the eps square after unmaking a pawn
        * moving two squares*/
        if self.hist[self.hdp].m.tipe == mv::MOVE_TYPE_PAWN_TWO {
            if side == color::WHITE {
                self.piece[self.hist[self.hdp].m.from - 8] = square::EMPTY;
                self.color[self.hist[self.hdp].m.from - 8] = square::EMPTY;
                self.en_passant = -1;
            }
            else {
                self.piece[self.hist[self.hdp].m.from + 8] = square::EMPTY;
                self.color[self.hist[self.hdp].m.from + 8] = square::EMPTY;
                self.en_passant = -1;
            }
        }

        /* Unmaking an en pasant capture */
        if self.hist[self.hdp].m.tipe == mv::MOVE_TYPE_EPS {

            if side == color::WHITE {
                /* The pawn */
                self.piece[self.hist[self.hdp].m.dest + 8] = pi::PAWN;
                self.color[self.hist[self.hdp].m.dest + 8] = color::BLACK;
                /* The eps square */
                self.piece[self.hist[self.hdp].m.dest] = square::EPS_SQUARE;
                self.en_passant = self.piece[self.hist[self.hdp].m.dest] as i16;
            }
            else {
                /* The pawn */
                self.piece[self.hist[self.hdp].m.dest - 8] = pi::PAWN;
                self.color[self.hist[self.hdp].m.dest - 8] = color::WHITE;
                /* The eps square */
                self.piece[self.hist[self.hdp].m.dest] = square::EPS_SQUARE;
                //            color[hist[hdp].m.dest] = EMPTY;
                self.en_passant = self.piece[self.hist[self.hdp].m.dest] as i16;
            }
            //self.en_passant = -1;
        }

        /* Undo Castle: return rook to its original square */
        if self.hist[self.hdp].m.tipe == mv::MOVE_TYPE_CASTLE {
            
            /* Take the rook to its poriginal place */
            if self.hist[self.hdp].m.dest == square::G1 && side == color::WHITE {
                self.piece[square::H1] = pi::ROOK;
                self.color[square::H1] = color::WHITE;
                self.piece[square::F1] = square::EMPTY;
                self.color[square::F1] = square::EMPTY;
            }
            else if self.hist[self.hdp].m.dest == square::C1 && side == color::WHITE {
                self.piece[square::A1] = pi::ROOK;
                self.color[square::A1] = color::WHITE;
                self.piece[square::D1] = square::EMPTY;
                self.color[square::D1] = square::EMPTY;
            }
            else if self.hist[self.hdp].m.dest == square::G8 && side == color::BLACK {
                self.piece[square::H8] = pi::ROOK;
                self.color[square::H8] = color::BLACK;
                self.piece[square::F8] = square::EMPTY;
                self.color[square::F8] = square::EMPTY;
            }
            else if self.hist[self.hdp].m.dest == square::C8 && side == color::BLACK {
                self.piece[square::A8] = pi::ROOK;
                self.color[square::A8] = color::BLACK;
                self.piece[square::D8] = square::EMPTY;
                self.color[square::D8] = square::EMPTY;
            }
        }
        //self.side = side;
    
        //self.ply -= 1;
        self.hist.pop();
        
    }
}