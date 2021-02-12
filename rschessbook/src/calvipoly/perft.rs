use std::time::{Instant};

use super::board;
use super::moves;


pub struct Perft {

}


impl Perft {
    pub fn perft (board: &mut board::Sboard, depth: i32) {
        let mut time: u128;
        let mut now = Instant::now();
        
        {
            println!("depth\ttime (MiliSecs.)\t\t\tnodes");
            println!("-----\t----------------\t\t\t-----");
            for i in 1..depth+1 {
                
                let nodes = Perft::mini_max(board, i);
                time = now.elapsed().as_millis();
                println!("{}\t{}\t\t\t\t\t{}", i, time, nodes);
                now = Instant::now();
                
                //board.clone().print_board();
            }
        }
        
        time = now.elapsed().as_millis();    // it throws u128
        println!("Time used: {}", time);
        
    }
    
    
    
    fn mini_max (board: &mut board::Sboard, depth: i32) -> u64 {
        let mut nodes: u64 = 0;

        if depth == 0 { return 1; }
        
        let mut moves: Vec<moves::Smove> = Vec::new();
        let num_moves = board.gen_moves(&mut moves);

        for i in 0..num_moves {     //moves.len() {
            let tmp = board.make_move(&mut moves[i as usize]);
            if tmp.is_none() { 
                board.undo_move(&mut moves[i as usize]);
                continue;
            }

            nodes += Perft::mini_max(board, depth-1);

            board.undo_move(&mut moves[i as usize]);
        }

        nodes
    }
    
    /*
    /* Returns the number of posible positions to a given depth. Based on the
    perft function on Danasah */
    pub fn mini_max (board: &mut board::Sboard, depth: i32) -> u64 {
        //int i;
        //int movecnt;			/* The number of available moves */
        let num_moves: i32;
        //unsigned long long nodes = 0;
        let mut nodes: u64 = 0;

        //if (!depth)
        //return 1;
        if depth == 0 { return 1; }

        //MOVE moveBuf[200];		/* List of movements */
        let mut moves: Vec<moves::Smove> = Vec::new();
        //let turn = board.side;
        /* Generate and count all moves for current position */
        //movecnt = GenMoves (side, moveBuf);
        num_moves = board.gen_moves(board.side, &mut moves);
//println!("82 rey está en e1 {} - {}", board.piece[4], board.color[4]);
//println!("------------------------");
        /* Once we have all the moves available, we loop through them */
        //for (i = 0; i < movecnt; ++i)
        for i in 0..moves.len() {       //num_moves {
            /* Not a legal move? Then we unmake it and continue to the next one in the list */
            //println!("82 -> from: {} ; to: {}", moves[i as usize].from, moves[i as usize].dest);
            let tmp = board.make_move(&mut moves[i as usize]);
            if tmp.is_none() {   //(!MakeMove (moveBuf[i])) {
                //println!("83 perft jugada es none");
                board.undo_move();
                panic!("jugada erronea");
                continue;
            }

            /* Just in case we want to count for checks */
        //        if (IsInCheck(side))
        //        {
        //            count_checks++;
        //        }
        //println!("nodes {} -> {} / {}", nodes, moves[i as usize].from, moves[i as usize].dest);
        //board.clone().print_board();
        //println!("turno {}", board.get_side());
            /* This 'if' takes us to the deep of the position */
            nodes += Perft::mini_max(board, depth-1);   //perft (depth - 1);
            board.undo_move();  //TakeBack ();
        //board.clone().print_board();
        //println!("turno {} - {} - {}", board.get_side(), i, num_moves);
        }

        return nodes;
    }
    */
}