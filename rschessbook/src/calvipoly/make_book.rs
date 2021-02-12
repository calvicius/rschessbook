use super::{book, pgn, board, util, san};

use std::time::{Instant};
//use std::io;
use std::io::prelude::*;
use std::fs::File;



// variables

static mut MAX_PLY: i32 = 1024;




pub fn make_new_book (pgn_file: &str, book_bin: &str, half_moves: i16) {
    //let half_moves: i16 = 20;
    let mut book: book::Sbook = book::Sbook::new();
    let mut final_book: Vec<book::SfinalEntry> = Vec::new();

    println!("inserting games ...");
    book_insert(pgn_file, &mut book, half_moves);
    println!("ending calculations ...");
    book.do_calculations();
    
    for (_key, value) in book.btree.iter_mut() {
        for mov in value.iter() {
            let tmp = book::SfinalEntry::new(mov.key, mov.move_, mov.weight, mov.learn);
            final_book.push(tmp);
        }
    }
    
    //let name: &str = "book.bin";
    let file = File::create(book_bin);

    println!("writing book file ...");
    match file {
        Ok(mut f) => {
            for entry in final_book.iter() {
                write_integer(&mut f,8,entry.key);
                write_integer(&mut f,2,entry.move_.into());
                write_integer(&mut f,2,entry.weight.into());
                write_integer(&mut f,4,0);
            }
        },
        Err(e) => panic!("error {} creating book file {}", e, book_bin),
    }
}



// book_insert()

fn book_insert(file_name: &str, book: &mut book::Sbook, half_moves: i16) {

    let mut pgn: pgn::Spgn = pgn::Spgn::new();
    let mut board: board::Sboard = board::Sboard::new();
    
    let mut ply: i32;
    let mut result: i32;
    let mut san: String = String::new();    //from("");
    //let mut move_ : i32;
    //let mut pos: i32;
    let mut count_moves: i16;

    assert_ne!(file_name.len(), 0);

    // init

    let time: u128;
    let now = Instant::now();

    pgn.init_number_game(1);
    // scan loop

    pgn.pgn_open(file_name);

    let mut contador: i32 = 0;
    while pgn.pgn_next_game() {
        board.set_fen(util::START_FEN);

        ply = 0;
        result = 0;

        let pgn_result = pgn.get_result();

        if pgn_result.trim() == "1-0" {
            result = 1;
        } 
        else if pgn_result.trim() == "0-1" {
            result = -1;
        }

        let max_ply: i32;
        unsafe {
            max_ply = MAX_PLY;
        }

        count_moves = 0;
        while pgn.pgn_next_move(&mut san, 256) {
            if ply < max_ply {
                
                if count_moves <= half_moves {
                    let san1 = san.clone();
                
                    let mov = san::move_from_san(san1.to_string(), &mut board);
                    if mov.is_none() {
                        let tmp = format!(
                            "book_insert(): illegal move \"{}\" at line {}, column {},game {}\n",
                            san, pgn.move_line, pgn.move_column, pgn.game_nb);
                        util::my_fatal(tmp.as_str());
                    }

                    let hash = mov.clone().unwrap().hash;
                    let mov_u16 = mov.clone().unwrap().encoded_move;
                    board.make_move(&mut mov.unwrap());

                    book.insert_move(hash, mov_u16, result);
                }              
            }
            count_moves += 1;
        }
        contador += 1;
        if contador % 500 == 0 { println!("until now {} games processed", contador); }
    }
    println!("total processed games: {}", contador);
    time = now.elapsed().as_secs().into();    // it throws u128
    println!("Time used: {}", time);
}


// write_integer()

fn write_integer(file: &mut File, size: usize, n: u64) {

    let mut b: u64;
 
    assert!(size > 0 && size <= 8);
    assert!(size==8 || n>>(size*8)==0);
    
    for i in (0..=size-1).rev() {
       b = (n >> (i*8)) & 0xFF;

       assert!(b<256);
       
       let b1: [u8; 1] = [b as u8];
       let _ = file.write(&b1);
    }
}