use super::board as b;
use super::color;
use super::square;
//use super::piece;
use super::moves as mv;



// move_from_san()

pub fn move_from_san(string_: String, mut board: &mut b::Sboard) -> Option<mv::Smove> {

    //let mut s: String = String::with_capacity(16);

    assert!(string_.len() > 0);

    //s = String::from("");

    let move_ = get_move_from_san(string_, &mut board);

    return move_;
}

// get_move_from_san()

fn get_move_from_san(san: String, mut board: &mut b::Sboard) -> Option<mv::Smove> {
    use regex::Regex;

    // we shall have 8 tokens
    let pattern: &str = r"^([PNBRQK])?([a-h])?([1-8])?(x|-)?([a-h][1-8])(=?[qrbnQRBN])?(\+|#)?$";
    
    let mut tokens: [&str; 8] = ["", "", "", "", "", "", "", ""];
    let pieces: [&str; 6] = ["", "N", "B", "R", "Q", "K"];
    let promotions: [char; 9] = [' ', ' ', ' ', ' ', ' ', 'Q', 'R', 'B', 'N'];
    let mut uci_str: String = String::from("");
    let from: &str;
    let to: &str;
    let mut moves: Vec<mv::Smove> = Vec::new();
    
    

    // FIRST: test if the move is castle

    match san.as_str() {
        "O-O" | "O-O+" => {
            if board.side == color::WHITE {
                from = "e1";    //"e1g1".to_string(); 
                to = "g1";
            }
            else {
                //uci_str = "e8g8".to_string(); 
                from = "e8";
                to = "g8";
            }
            
            //res = moveUCI(board, uciStr)
            //return
            let res = get_move_from_uci(from, to, &mut board);
            return res;
        },
        "O-O-O" | "O-O-O+" => {
            if board.side == color::WHITE {
                //uci_str = "e1c1".to_string();
                from = "e1";
                to = "c1";
            }
            else {
                //uci_str = "e8c8".to_string();
                from = "e8";
                to = "c8";
            }
            
            let res = get_move_from_uci(from, to, &mut board);
            return res;
        }
        _ => (),
    }

    // SECOND: begin with regex of SAN move

    let re = Regex::new(pattern).unwrap();

    assert!(re.is_match(&san));

    for caps in re.captures_iter(&san) {
        tokens[0] = caps.get(0).unwrap().as_str();
        if caps.get(1).is_some() {
            tokens[1] = caps.get(1).unwrap().as_str();
        }
        if caps.get(2).is_some() {
            tokens[2] = caps.get(2).unwrap().as_str();
        }
        if caps.get(3).is_some() {
            tokens[3] = caps.get(3).unwrap().as_str();
        }
        if caps.get(4).is_some() {
            tokens[4] = caps.get(4).unwrap().as_str();
        }
        if caps.get(5).is_some() {
            tokens[5] = caps.get(5).unwrap().as_str();
        }
        if caps.get(6).is_some() {
            tokens[6] =caps.get(6).unwrap().as_str();
        }
        if caps.get(7).is_some() {
            tokens[7] = caps.get(7).unwrap().as_str();
        }
    }
    /*
    ! Eaxmple with largest possible SAN = Nf3xd4=Q+ (len=9)
    ! tokens(0) = Nf3xd4=Q+
    ! tokens(1) = N
    ! tokens(2) = f
    ! tokens(3) = 3
    ! tokens(4) = x
    ! tokens(5) = d4
    ! tokens(6) = =Q
    ! tokens(7) = +
    */

    // THIRD, test if origin square is complete

    if tokens[2].len() > 0 && tokens[3].len() > 0 {
        //uciStr = tokens(2)(1:1) // tokens(3)(1:1) // tokens(5)(1:2)
        uci_str.clear();
        uci_str.push_str(tokens[2]);
        uci_str.push_str(tokens[3]);
        //uci_str.push_str(tokens[5]);

        if tokens[6].len() > 0 {
            // convert to lower case promotion piece :Q, B, ...
            let prom: char = tokens[6].chars().nth(1).unwrap();
            uci_str.push(prom.to_lowercase().nth(0).unwrap());
        }
        
        //res = moveUCI(board, uciStr)
        //return
        let res = get_move_from_uci(uci_str.trim(), tokens[5].trim(), &mut board);
        return res;
    }

    // FOURTH, the other moves

    // variables to find the correct move
    //let str_san_length = san.trim().len();
    let piece_san: usize = pieces.iter().position(|&r| r == tokens[1].trim()).unwrap();    // PAWN WILL BE ZERO
    
    let num_moves: i32 = board.gen_moves(&mut moves);
    //let mut move_num = 0; // 0 = NONE_MOVE

    for j in 0..num_moves {
        //tmp_move = moves[j as usize].clone();
        let mvs_from: usize = moves[j as usize].from;
        let mvs_to: usize   = moves[j as usize].dest;

        let mut end_str: String = String::from("");
        let tmp_str: String     = mv::get_uci_format(moves[j as usize].encoded_move);

        // first examine the destination square
        if square::ALGEBRA_SQUARE[mvs_to] == tokens[5] {
            
            // same piece
            if board.piece[mvs_from] == piece_san {
                
                // promotion?
                if tokens[6].len() > 0 {
                    let mut prom: char;
                    // some notations ommits the '='
                    if tokens[6].chars().nth(0).unwrap() == '=' {
                        prom = tokens[6].chars().nth(1).unwrap();
                    }
                    else { prom = tokens[6].chars().nth(0).unwrap(); }

                    let promoted: usize = promotions.iter().position(|&r| r == prom).unwrap();
                    //convert to lowercase vias ascii codes
                    prom = ((prom as u8) + 32) as char;
                    if promoted == moves[j as usize].tipe {
                        end_str.push_str(tokens[5].trim());
                        end_str.push(prom);
                    }
                }
                else {
                    end_str.push_str(tokens[5].trim());
                }

                // disambiguate?

                if tokens[2].trim().len() > 0 || tokens[3].trim().len() > 0 {

                    if tokens[2].trim().len() > 0 && tokens[3].trim().len() == 0 {
                        if tmp_str[0..1].trim() == tokens[2].trim() {
                            if tmp_str[2..].trim() == end_str.trim() {
                                return Some(moves[j as usize].clone());
                            }
                        }
                    }
                    if tokens[3].trim().len() > 0 && tokens[2].trim().len() == 0 {
                        if &tmp_str[1..2] == tokens[3].trim() {
                            if tmp_str[2..].trim() == end_str.trim() {
                                return Some(moves[j as usize].clone());
                            }
                        }
                    }

                }

                // no disambiguation
                if tokens[2].len() == 0 && tokens[3].len() == 0 {

                    if tmp_str[2..].trim() == end_str.trim() {
                        return Some(moves[j as usize].clone());
                    }
                }
                
            }
        }
    }
    None
}


fn get_move_from_uci(v_from: &str, v_to: &str, board: &mut b::Sboard) -> Option<mv::Smove> {

    let mut moves: Vec<mv::Smove> = Vec::new();

    let from: usize = square::ALGEBRA_SQUARE.iter().position(|&r| r == v_from).unwrap();
    let to: usize = square::ALGEBRA_SQUARE.iter().position(|&r| r == v_to).unwrap();

    let num_moves: i32 = board.gen_moves(&mut moves);

    for j in 0..num_moves {
        //tmp_move = moves[j as usize].clone();
        //let mvs_from: usize = moves[j as usize].from;
        //let mvs_to: usize   = moves[j as usize].dest;

        if from == moves[j as usize].from && 
                to == moves[j as usize].dest {
            return Some(moves[j as usize].clone());
        }
    }

    None
}