use std::io::prelude::*;
use std::fs::File;

use super::util;

const PGN_STRING_SIZE: i32 = 256;

// constants

const DISP_MOVE: bool = false;
const DISP_TOKEN: bool = false;
const DISP_CHAR: bool = false;

const TAB_SIZE: i32 = 8;

const CHAR_EOF: i32 = 256;

// types
#[derive(PartialEq, Debug)]
pub enum TokenT {
    TokenError   = -1,
    TokenEof     = 256,
    TokenSymbol  = 257,
    TokenString  = 258,
    TokenInteger = 259,
    TokenNag     = 260,
    TokenResult  = 261
}



pub struct Spgn {
    file: Option<std::io::Result<File>>,

    char_hack: i32,
    char_line: i32,
    char_column: i32,
    char_unread: bool,
    char_first: bool,

    token_type: i32,
    token_string: String,
    token_length: i32,
    token_line: i32,
    token_column: i32,
    token_unread: bool,
    token_first: bool,

    result: String,
    fen: String,

    pub move_line: i32, 
    pub move_column: i32,
    pub game_nb: i32,
} 


impl Spgn {
    pub fn new() -> Self {
        Spgn {
            file: None,

            char_hack: CHAR_EOF,
            char_line: 1,
            char_column: 0,
            char_unread: false,
            char_first: true,

            token_type: TokenT::TokenError as i32,
            token_string: String::with_capacity(PGN_STRING_SIZE as usize),
            token_length: -1,
            token_line: -1,
            token_column: -1,
            token_unread: false,
            token_first: true,

            result: String::with_capacity(PGN_STRING_SIZE as usize),
            fen: String::with_capacity(PGN_STRING_SIZE as usize),

            move_line: -1,
            move_column: -1,
            game_nb: 0, 
        }
    }


    // init number of game

    pub fn init_number_game(&mut self, number: i32) {
        self.game_nb = number;
    }

    // get reult of the game

    pub fn get_result(&mut self) -> String {
        self.result.clone()
    }


    // pgn_open()

    pub fn pgn_open(&mut self, file_name: &str) {

        //ASSERT(pgn!=NULL);
        assert_ne!(file_name.len(), 0);

        //pgn->file = fopen(file_name,"r");
        let file = File::open(file_name);
        if file.is_err() {
            let tmp = format!("pgn_open(): can't open file \"{}\"\n",file_name);
            util::my_fatal(tmp.as_str());
        }
        self.file = Some(file);

        self.char_hack = CHAR_EOF; // DEBUG
        self.char_line = 1;
        self.char_column = 0;
        self.char_unread = false;
        self.char_first = true;

        self.token_type = TokenT::TokenError as i32; // DEBUG
        self.token_string = String::from("?"); // DEBUG
        self.token_length = -1; // DEBUG
        self.token_line = -1; // DEBUG
        self.token_column = -1; // DEBUG
        self.token_unread = false;
        self.token_first = true;

        self.result = String::from("?"); // DEBUG
        self.fen = String::from("?"); // DEBUG

        self.move_line = -1; // DEBUG
        self.move_column = -1; // DEBUG
    }


    // pgn_next_game()

    pub fn pgn_next_game(&mut self) -> bool {

        let mut name: String;
        let mut value: String;
    
        // init
    
        self.result = String::from("*");
        self.fen = String::from("");
    
        // loop
    
        loop {
            self.pgn_token_read();
            if self.token_type as u8 as char != '[' { break; }

            // tag
            
            self.pgn_token_read();
            if self.token_type != TokenT::TokenSymbol as i32 {
                let tmp = format!(
                    "pgn_next_game(): malformed tag at line {}, column {}, game {}\n",
                    self.token_line, self.token_column, self.game_nb);
                util::my_fatal(tmp.as_str());
            }
            
            name = self.token_string.clone();
    
            self.pgn_token_read();
            if self.token_type != TokenT::TokenString as i32 {
                let tmp = format!(
                    "pgn_next_game(): malformed tag at line {}, column {}, game {}\n",
                    self.token_line, self.token_column, self.game_nb);
                util::my_fatal(tmp.as_str());
            }
            
            value = self.token_string.clone();
    
            self.pgn_token_read();
            if self.token_type as u8 as char != ']' {
                let tmp = format!(
                    "pgn_next_game(): malformed tag at line {}, column {}, game {}\n",
                    self.token_line, self.token_column, self.game_nb);
                util::my_fatal(tmp.as_str());
            }

            // special tag?
            
            if name == "Result" {
                self.result = value.to_string();
            } 
            else if name == "FEN" {
                self.fen = value.to_string();
            }
        }
        
        if self.token_type == TokenT::TokenEof as i32 { 
            return false; 
        }
        
        self.pgn_token_unread();
        
        true
    }


    // pgn_token_read()

    fn pgn_token_read(&mut self) {
    
        // token "stack"
        
        if self.token_unread {
            self.token_unread = false;
            return;
        }
        
        // consume the current token
    
        if self.token_first {
            self.token_first = false;
        } 
        else {
            assert!(self.token_type !=TokenT::TokenError as i32);
            assert!(self.token_type !=TokenT::TokenEof as i32);
        }
    
        // read a new token
    
        self.pgn_read_token();
        
        if self.token_type == TokenT::TokenError as i32 {
            let tmp = format!(
                "pgn_token_read(): lexical error at line {}, column {}, game {}\n",
                self.char_line, self.char_column, self.game_nb);
            util::my_fatal(tmp.as_str());
            //my_fatal("pgn_token_read(): lexical error at line %d, column %d, game %d\n",pgn->char_line,pgn->char_column,pgn->game_nb);
        }
        if DISP_TOKEN {
            println!("< {} C{} \"{}\" ({})\n",
                self.token_line, self.token_column, self.token_string, self.token_type);
        }
    }


    // pgn_token_unread()

    fn pgn_token_unread(&mut self) {
        
        assert!(!self.token_unread);
        assert!(!self.token_first);
        assert!(!self.token_unread);
        assert!(!self.token_first);
    
        self.token_unread = true;
    }


    // pgn_read_token()

    fn pgn_read_token (&mut self) {

        //let c: u8 = 0;
        // skip white-space characters
    
        self.pgn_skip_blanks();
    
        // init
        
        self.token_type = TokenT::TokenError as i32;
        self.token_string = String::from("");
        self.token_length = 0;
        self.token_line = self.char_line;
        self.token_column = self.char_column;
    
        // determine token type
        
        if self.char_hack == CHAR_EOF {
    
            self.token_type = TokenT::TokenEof as i32;
    
        }
        else if ".[]()<>".contains(self.char_hack as u8 as char) {
            // single-character token
            self.token_type = self.char_hack;
            //sprintf(pgn->token_string,"%c",pgn->char_hack);
            self.token_string.clear();
            self.token_string.push(self.char_hack as u8 as char);
            self.token_length = 1;
    
        } 
        else if self.char_hack as u8 as char == '*' {
    
            self.token_type = TokenT::TokenResult as i32;
            self.token_string.clear();
            self.token_string.push(self.char_hack as u8 as char);
            self.token_length = 1;
    
        } 
        else if self.char_hack as u8 as char == '!' {
    
            self.pgn_char_read();
            
            if self.char_hack as u8 as char == '!' { // "!!"
        
                self.token_type = TokenT::TokenNag as i32;
                self.token_string.clear();
                self.token_string.push('3');
                self.token_length = 1;
        
            } 
            else if self.char_hack as u8 as char == '?' { // "!?"
        
                self.token_type = TokenT::TokenNag as i32;
                self.token_string.clear();
                self.token_string.push('5');
                self.token_length = 1;
        
            } 
            else { // "!"
        
                self.pgn_char_unread();
        
                self.token_type = TokenT::TokenNag as i32;
                self.token_string.clear();
                self.token_string.push('1');
                self.token_length = 1;
            }
    
        }
        else if self.char_hack as u8 as char == '?' {
    
            self.pgn_char_read();
    
            if self.char_hack as u8 as char == '?' { // "??"
        
                self.token_type = TokenT::TokenNag as i32;
                self.token_string.clear();
                self.token_string.push('4');
                self.token_length = 1;
        
            } 
            else if self.char_hack as u8 as char == '!' { // "?!"
        
                self.token_type = TokenT::TokenNag as i32;
                self.token_string.clear();
                self.token_string.push('6');
                self.token_length = 1;
        
            } 
            else { // "?"
        
                self.pgn_char_unread();
        
                self.token_type = TokenT::TokenNag as i32;
                self.token_string.clear();
                self.token_string.push('2');
                self.token_length = 1;
            }
        
        } 
        else if is_symbol_start(self.char_hack) {
            
            // symbol, integer, or result
        
            self.token_type = TokenT::TokenInteger as i32;
            self.token_length = 0;
    
            loop {
    
                if self.token_length >= PGN_STRING_SIZE-1 {
                    let tmp = format!(
                        "pgn_read_token(): symbol too long at line {}, column {},game {}\n",
                        self.char_line,
                        self.char_column,
                        self.game_nb);
                    util::my_fatal(tmp.as_str());
                }
        
                if !(self.char_hack as u8 as char).is_digit(10) { 
                    self.token_type = TokenT::TokenSymbol as i32; 
                }
        
                //pgn->token_string[pgn->token_length++] = pgn->char_hack;
                self.token_string.push(self.char_hack as u8 as char);
                self.token_length += 1;

                self.pgn_char_read();

                if !is_symbol_next(self.char_hack) { break; }
            }
    
            self.pgn_char_unread();
        
            assert!(self.token_length > 0 && self.token_length < PGN_STRING_SIZE);
            /*
            pgn->token_string[pgn->token_length] = '\0';
        
            if (my_string_equal(pgn->token_string,"1-0")
                || my_string_equal(pgn->token_string,"0-1")
                || my_string_equal(pgn->token_string,"1/2-1/2")) {
                pgn->token_type = TOKEN_RESULT;
            }
            */
            if self.token_string == "1-0" ||
                    self.token_string == "0-1" ||
                    self.token_string == "1/2-1/2" {

                self.token_type = TokenT::TokenResult as i32;
            }
    
        }
        else if self.char_hack as u8 as char == '"' {
    
        // string
    
        self.token_type = TokenT::TokenString as i32;
        self.token_length = 0;
    
        loop {
    
                self.pgn_char_read();
        
                if self.char_hack == CHAR_EOF {
                    let tmp = format!(
                        "pgn_read_token(): EOF in string at line {}, column {}, game {}\n",
                        self.char_line,
                        self.char_column,
                        self.game_nb);
                    util::my_fatal(tmp.as_str());
                }
        
                if self.char_hack as u8 as char == '"' { break; }
        
                if self.char_hack as u8 as char == '\\' {
        
                    self.pgn_char_read();
        
                    if self.char_hack == CHAR_EOF {
                        let tmp = format!(
                            "pgn_read_token(): EOF in string at line {}, column {}, game {}\n",
                            self.char_line, self.char_column, self.game_nb);
                        util::my_fatal(tmp.as_str());
                    }
        
                    if self.char_hack as u8 as char != '"' && 
                            self.char_hack as u8 as char != '\\' {
        
                        // bad escape, ignore
        
                        if self.token_length >= PGN_STRING_SIZE-1 {
                            let tmp = format!(
                                "pgn_read_token(): string too long at line {}, column {},game {}\n",
                                self.char_line, self.char_column, self.game_nb);
                            util::my_fatal(tmp.as_str());
                        }
        
                        //pgn.token_string[pgn->token_length++] = '\\';
                        self.token_string.push(self.char_hack as u8 as char);
                        self.token_length += 1;
                    }
                }
        
                if self.token_length >= PGN_STRING_SIZE-1 {
                    let tmp = format!(
                        "pgn_read_token(): string too long at line {}, column {},game {}\n",
                        self.char_line, self.char_column, self.game_nb);
                    util::my_fatal(tmp.as_str());
                }
        
                //pgn->token_string[pgn->token_length++] = pgn->char_hack;
                self.token_string.push(self.char_hack as u8 as char);
                self.token_length += 1;
            }
        
            assert!(self.token_length >= 0 && self.token_length < PGN_STRING_SIZE);
        }
        else if self.char_hack as u8 as char == '$' {
    
            // NAG
        
            self.token_type = TokenT::TokenNag as i32;
            self.token_length = 0;
        
            loop {
        
                self.pgn_char_read();
        
                if !(self.char_hack as u8 as char).is_digit(10) { break; }
        
                if self.token_length >= 3 {
                    let tmp = format!(
                        "pgn_read_token(): NAG too long at line {}, column {}, game {}\n",
                        self.char_line, self.char_column, self.game_nb);
                    util::my_fatal(tmp.as_str());
                }
        
                //pgn->token_string[pgn->token_length++] = pgn->char_hack;
                self.token_string.push(self.char_hack as u8 as char);
                self.token_length += 1;
            }
        
            self.pgn_char_unread();
        
            if self.token_length == 0 {
                let tmp = format!(
                    "pgn_read_token(): malformed NAG at line {}, column {},game {}\n",
                    self.char_line, self.char_column, self.game_nb);
                util::my_fatal(tmp.as_str());
            }
        
            //ASSERT(pgn->token_length>0&&pgn->token_length<=3);
            //pgn->token_string[pgn->token_length] = '\0';
            assert!(self.token_length > 0 && self.token_length <= 3);
        } 
        else {
            // unknown token
            let tmp = format!(
                "lexical error at line {}, column {}, game {}\n",
                self.char_line, self.char_column, self.game_nb);
            util::my_fatal(tmp.as_str());
        }
        
    }


    // pgn_skip_blanks()

    fn pgn_skip_blanks(&mut self) {

        loop {
            self.pgn_char_read();
            
            if self.char_hack==CHAR_EOF { 
                break;
            } 
            else if char::is_ascii_whitespace(&(self.char_hack as u8 as char)) {
        
                // skip white space
        
            }
            else if self.char_hack as u8 as char == ';' {
        
                // skip comment to EOL
        
                loop {
                    self.pgn_char_read();
        
                    if self.char_hack == CHAR_EOF {
                        let tmp = format!(
                            "pgn_skip_blanks(): EOF in comment at line {}, column {},game {}\n",
                            self.char_line,
                            self.char_column,
                            self.game_nb);
                        util::my_fatal(tmp.as_str());
                    }
                    
                    if self.char_hack as u8 as char == '\n' {
                        break;
                    }
                } //while (pgn->char_hack != '\n');
        
            } 
            else if self.char_hack as u8 as char == '%' && self.char_column == 0 {
        
                // skip comment to EOL
        
                loop {
        
                    self.pgn_char_read();
        
                    if self.char_hack == CHAR_EOF {
                        let tmp = format!(
                            "pgn_skip_blanks(): EOF in comment at line {}, column {}, game {}\n",
                            self.char_line,
                            self.char_column,
                            self.game_nb);
                        util::my_fatal(tmp.as_str());
                    }
                    if self.char_hack as u8 as char == '\n' {
                        break;
                    }
                } //while (pgn->char_hack != '\n');
        
            }
            else if self.char_hack as u8 as char == '{' {
        
                // skip comment to next '}'
        
                loop {
        
                    self.pgn_char_read();
        
                    if self.char_hack == CHAR_EOF {
                        let tmp = format!(
                            "pgn_skip_blanks(): EOF in comment at line {}, column {}, game {}\n",
                            self.char_line,
                            self.char_column,
                            self.game_nb);
                        util::my_fatal(tmp.as_str());
                    }
                    if self.char_hack as u8 as char == '}' { break; }
                } 
        
            } 
            else { // not a white space
        
                break;
            }
        }
    }


    // pgn_char_read()

    fn pgn_char_read(&mut self) {
    
        // char "stack"
    
        if self.char_unread {
            self.char_unread = false;
            return;
        }
    
        // consume the current character
    
        if self.char_first {
            self.char_first = false;
        } 
        else {
            // update counters
        
            assert_ne!(self.char_hack, CHAR_EOF);
            
            if self.char_hack as u8 as char == '\n' {
                self.char_line += 1;
                self.char_column = 0;
            } 
            else if self.char_hack as u8 as char == '\t' {
                self.char_column += TAB_SIZE - (self.char_column % TAB_SIZE);
            } else {
                self.char_column += 1;
            }
        }
    
        // read a new character
    
        let mut buffer = [0; 1];	//only un char
        
        if let Some(file1) = self.file.as_ref() {
            let file2 = file1.as_ref();
            if file2.is_ok() {		        //file2 is a Result<>
                match file2 {
                    Ok(mut f) => {
                        let _ = f.read_exact(&mut buffer);
                        self.char_hack = buffer[0].into();
                    },
                    Err(_) => {},
                }
                // char_hack == 0 end of file
            }
            else if let Err(e) = file2 {
                let tmp = format!("Error reading char in pgn_char_read : {}", e);
                self.char_hack = CHAR_EOF;
                util::my_fatal(tmp.as_str());
            }
        }

        // end of file
        if self.char_hack == 0 {
            // just above here is the implementation
            
            println!("End of file");
            self.char_hack = CHAR_EOF;
        }
        
        if DISP_CHAR {
            println!("< {} C{} '{}' ({})",
                self.char_line,
                self.char_column,
                self.char_hack,
                self.char_hack);
        }
    }


    // pgn_char_unread()

    fn pgn_char_unread(&mut self) {

        assert!(!self.char_unread);
        assert!(!self.char_first);

        self.char_unread = true;
    }


    // pgn_next_move()

    pub fn pgn_next_move(&mut self, string_: &mut String, size: i32) -> bool {

        let mut depth: i32;
    
        assert!(size >= PGN_STRING_SIZE);
    
        // init
    
        self.move_line = -1; // DEBUG
        self.move_column = -1; // DEBUG
    
        // loop
    
        depth = 0;
    
        loop {
    
            self.pgn_token_read();

            if self.token_type as u8 as char == '(' {
        
                // open RAV
        
                depth += 1;
        
            } 
            else if self.token_type as u8 as char == ')' {
        
                // close RAV

                if depth == 0 {
                    let tmp = format!(
                        "pgn_next_move(): malformed variation at line {}, column {}, game {}\n",
                        self.token_line, self.token_column, self.game_nb);
                    util::my_fatal(tmp.as_str());
                }
        
                depth -= 1;
                assert!(depth>=0);
        
            } 
            else if self.token_type == TokenT::TokenResult as i32 {
        
                // game finished
        
                if depth > 0 {
                    let tmp = format!(
                        "pgn_next_move(): malformed variation at line {}, column {}, game {}\n",
                        self.token_line, self.token_column, self.game_nb);
                    util::my_fatal(tmp.as_str());
                }
        
                return false;
        
            } 
            else {
        
                // skip optional move number
        
                if self.token_type == TokenT::TokenInteger as i32 {
                    loop {
                        self.pgn_token_read(); 
                        if self.token_type as u8 as char != '.' { break; }
                    } //while (pgn->token_type == '.');
                }

                // move must be a symbol
        
                if self.token_type != TokenT::TokenSymbol as i32 {
                    let tmp = format!(
                        "pgn_next_move(): malformed move at line {}, column {}, game {}\n",
                        self.token_line, self.token_column, self.game_nb);
                    util::my_fatal(tmp.as_str());
                }
        
                // store move for later use
        
                if depth == 0 {

                    if self.token_length >= size {
                        let tmp = format!(
                            "pgn_next_move(): move too long at line {}, column {}, game {}\n",
                            self.token_line, self.token_column, self.game_nb);
                        util::my_fatal(tmp.as_str());
                    }
        
                    //strcpy(string,pgn->token_string);
                    *string_ = self.token_string.clone();

                    self.move_line = self.token_line;
                    self.move_column = self.token_column;
                }
        
                // skip optional NAGs
        
                loop {
                    self.pgn_token_read(); 
                    if self.token_type != TokenT::TokenNag as i32 { break; }
                } //while (pgn->token_type == TOKEN_NAG);

                self.pgn_token_unread();

                // return move
        
                if depth == 0 {
                    if DISP_MOVE {
                        println!("move=\"{}\"",string_);
                    }
                    return true;
                }
            }
        }
    
        //false
    }
}



// functions out of struct methods

// is_symbol_start()

fn is_symbol_start(c: i32) -> bool {
    let test = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let index = test.find(c as u8 as char);
    match index {
        Some(_) => {
            return true;
        },
        None => return false,
    }
}


// is_symbol_next()

fn is_symbol_next(c: i32) -> bool {

    let test = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789_+#=:-/";
    let index = test.find(c as u8 as char);
    match index {
        Some(_) => {
            return true;
        },
        None => return false,
    }
}