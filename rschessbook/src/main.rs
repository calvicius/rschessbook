#![allow(dead_code)]

mod calvipoly;

use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    calvipoly::my_main(args);
}
