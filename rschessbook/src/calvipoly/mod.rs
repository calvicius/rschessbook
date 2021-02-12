extern crate regex;

mod piece;
mod square;
mod color;
mod util;
mod moves;
mod board;
mod perft;
mod zobrist;
mod san;
mod pgn;
mod book;
mod make_book;


const HELP_MESSAGE: &str = r#"
SYNTAX
* rschessbook make-book [-pgn inputfile] [-bin outputfile] [-max-ply ply]
*
* if -bin parameter is omitted then the name book.bin will be created
* if -max-ply is omitted then 20 half-moves will be assigned
"#;

pub fn my_main(args: Vec<String>) {

    if args.len() < 2 {
        println!("{}",HELP_MESSAGE);
        std::process::exit(util::EXIT_SUCCESS);
    }

    if args.len() == 2 && (
            &args[1] == "help"   || 
            &args[1] == "-help"  || 
            &args[1] == "--help" ||  
            &args[1] == "-h"     ||  
            &args[1] == "/?" ) {
        println!("{}",HELP_MESSAGE);
        std::process::exit(util::EXIT_SUCCESS);
    }

    if args.len() >= 2 && args[1] =="make-book" {
        check_args(args.len(), args);
    }
}


pub fn check_args(argc: usize, argv: Vec<String>) {

    let mut i: usize = 0;
    let mut pgn_file: &str;
    let mut bin_file: &str;
    let mut half_moves: i16 = 20;

    pgn_file = "book.pgn";
    bin_file = "book.bin";

    while i < argc {
        if i == 0 {
            //pass
        }
        else if argv[i] == "make-book"{
            // skip
        }
        else if argv[i] == "-pgn" {
            i += 1;
            if i >= argc {
                util::my_fatal("book_make() -pgn : missing argument\n");
            }
            pgn_file = argv[i].as_str();
        }
        else if argv[i] == "-bin" {
            i += 1;
            if i >= argc {
                util::my_fatal("book_make() -bin : missing argument\n");
            }
            bin_file =argv[i].as_str();
        }
        else if argv[i] == "-max-ply" {
            i += 1;
            if i >= argc {
                util::my_fatal("book_make() -max-ply : missing argument\n");
            }
            let res = argv[i].parse::<i16>();
            if res.is_ok() {
                half_moves = res.unwrap();
            }
            else {
                util::my_fatal("book_make() -max-ply : numerical argument\n");
            }
            assert!(half_moves >= 0);
        }
        else {
            let tmp = format!("book_make(): unknown option \"{}\"\n",argv[i]);
            util::my_fatal(tmp.as_str());
        }
        i += 1;
    }

    make_book::make_new_book(pgn_file, bin_file, half_moves);
}



mod tests {
    use super::*;

    #[test]
    fn test_perft() {
        let depth: i32 = 4;
        let mut board1: board::Sboard = board::Sboard::new();
        perft::Perft::perft(&mut board1, depth);
    }

    #[test]
    fn test_valid_game() {
        let partida: Vec<&str> = vec![
        "d2-d4", "Nf6", "c4", "e6", "Nc3", "d5", "cxd5", "exd5", "Bg5", "c6", "e3", "Bf5", "Qf3", "Bg6",
        "Bxf6", "Qxf6", "Qxf6", "gxf6", "Nf3", "Nd7", "O-O-O", "Nb6", "Bd3", "Bb4", "Nh4", "Nc8",
        "Ne2", "Nd6", "Ng3", "O-O-O", "Nhf5", "Nxf5", "Nxf5", "Bxf5", "Bxf5+", "Kc7", "h4", "h6",
        "h5", "a5", "Rd3", "a4", "a3", "Be7", "Re1", "Rhe8", "Kc2", "Bd6", "Rc1", "Ra8", "Kb1", "b5",
        "f3", "b4", "axb4", "Bxb4", "e4", "Kd6", "Rc2", "Ra6", "Re3", "Ra7", "Rxc6+", "Kxc6", "exd5+",
        "Kxd5", "Rxe8", "Kxd4", "Rh8", "Bd2", "Bc2", "Ra5", "Rd8+", "Ke3", "Rd6", "a3", "Rd3+", "Ke2",
        "bxa3", "Rxh5", "Rd7", "Bc3", "Ka2", "Rc5", "Kb3", "h5", "Rxf7", "Kf2", "Rh7", "Kxg2",
        "Bg6", "Kxf3", "Rxh5", "Be5", "Kb4", "Rc1", "a4", "Kg4", "Rh7"
        ];

        let mut board: board::Sboard = board::Sboard::new();

        for jug in partida.iter() {
            let resul = san::move_from_san(jug.to_string(), &mut board);
            match resul {
                Some(mut mov) => {
                    board.make_move(&mut mov);
                    let fen = board.get_fen();
                    println!("fen {}", fen);
                },
                None => {
                    let tmp = format!("erroneous move in {}", jug);
                    panic!(tmp);
                },
            };
        }
    }

    #[test]
    fn test_set_get_fen() {
        let mut board: board::Sboard = board::Sboard::new();
        let mut move_buf: Vec<moves::Smove> = Vec::new();
        let num_jugadas: i32;
        //let side = board.get_side();

        let fen: &str = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";

        let resul = board.set_fen(fen);
        assert!(resul);

        move_buf.clear();
        num_jugadas = board.gen_moves (&mut move_buf);
        for i in 0..num_jugadas {
            println!("move {} -> {} / {}", i, move_buf[i as usize].from, 
                move_buf[i as usize].dest);
        }
        let resul = board.make_move(&mut move_buf[1]);
        match resul {
            Some(m) => {
                
                println!("from {} - {}", 
                    m.from, m.dest);
                
                let fen = board.get_fen();
                println!("fen {}", fen);
            },
            None => {panic!("invalid move");},
        };

        board.undo_move(&mut move_buf[1]);
        let resul = board.make_move(&mut move_buf[3]);
        match resul {
            Some(m) => {
                
                println!("from {} - {}", 
                    m.from, m.dest);
                
                let fen = board.get_fen();
                println!("fen {}", fen);
            },
            None => {panic!("invalid move");},
        };
    }
}