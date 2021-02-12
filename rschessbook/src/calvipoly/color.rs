pub const WHITE: usize = 0;
pub const BLACK: usize = 1;


// colour_is_ok()

pub fn colour_is_ok(colour: usize) -> bool {

    colour == BLACK || colour == WHITE
}


// colour_is_black()

pub fn colour_is_black(colour: usize) -> bool {

    assert!(colour_is_ok(colour));
 
    return colour == BLACK;
}


// colour_is_white()

pub fn colour_is_white(colour: usize) -> bool {

    assert!(colour_is_ok(colour));
 
    colour == WHITE
}