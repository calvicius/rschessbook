/*
*    Castling moves are represented somewhat unconventially as follows 
*    (this convention is related to Chess960).
*
*    white short      e1h1
*    white long       e1a1
*    black short      e8h8
*    black long       e8a8
*
*/

pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const RANK_MAX: usize = 7;
pub const RANK_MIN: usize = 0;
pub const FILE_MAX: usize = 7;
pub const FILE_MIN: usize = 0;

/*
pub enum Columnas { FILEA = 0, FILEB = 1, FILEC = 2, FILED = 3, FILEE = 4, 
    FILEF = 5, FILEG = 6, FILEH = 7, FILENONE = 8, }
pub enum Filas { RANK1 = 0, RANK2 = 1, RANK3 = 2, RANK4 = 3, RANK5 = 4, 
    RANK6 = 5, RANK7 = 6, RANK8 = 7, RANKNONE = 8, }
*/


pub fn get_col(pos: usize) -> usize {
    pos & 7
}


pub fn get_row(pos: usize) -> usize {
    pos >> 3    // must be unsigned
}



/***************************************
*    from here on, we will deal with the issues of reading 
*    the pgn file and creating the book
****************************************/

use std::io;
use std::io::Write;
use std::fs::File;

// constants

pub const EXIT_SUCCESS: i32 = 0;
pub const EXIT_FAILURE: i32 = 1;

// variables

static mut ERROR: bool = false;
static mut LOG_FILE: Option<io::Result<File>> = None;



// my_fatal()

pub fn my_fatal(format: &str) {

    let tmp = format!("RSCHESSBOOK {}\n", format);
    my_log(tmp.as_str());
    // This should be gui_send but this does not work.
    // Why?

    println!("tellusererror RSCHESSBOOK: {}", format);
    unsafe {
        if ERROR { // recursive error
            my_log("RSCHESSBOOK *** RECURSIVE ERROR ***\n");
            std::process::exit(EXIT_FAILURE);
                // abort();
        } 
        else {
            ERROR = true;
            quit();
        }
    }
}


// my_log()

pub fn my_log(text: &str) {
    
    unsafe {
        match &LOG_FILE {
            Some(somefile) => {
                match somefile {
                    Ok(file) => {
                        let mut f = file;
                        f.write_all(text.as_bytes()).expect("unable to write");
                    },
                    Err(_) => {},        
                }
            },
            None => {
                println!("{}", text);
            },
        }
    }
}


// quit()

pub fn quit() {
    my_log("RSCHESSBOOK *** QUIT ***");
    
    my_log("RSCHESSBOOK Calling exit\n");
    std::process::exit(EXIT_SUCCESS);
}
