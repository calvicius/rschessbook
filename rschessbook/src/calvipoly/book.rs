use std::collections::BTreeMap;


#[derive(Debug)]
pub struct SfinalEntry {
    pub key   : u64,
    pub move_ : u16, 
    pub weight: u16,
    pub learn : u32,
}

impl SfinalEntry {
    pub fn new (key:u64, move_:u16, weight:u16, learn:u32) -> Self {
        SfinalEntry {
            key   : key,
            move_ : move_, 
            weight: weight,
            learn : learn,
        }
    }
}



#[derive(Debug)]
pub struct Sentry {
    /* this 4 fields are mandatory */
    pub key   : u64,
    pub move_ : u16, 
    pub weight: u16,    // 2*(win_white)+(draws);  draws = total_games - win_white - win_black)
    pub learn : u32,    // not used
    /* this others are used for calculations */
    win_white: i32,
    win_black: i32,
    total_games: i32,
}
 
 impl Sentry {
    pub fn new() -> Self {
        Sentry {
            key: 0,
            move_: 0,
            weight: 0,
            learn: 0,
            win_white: 0,
            win_black: 0,
            total_games: 0,
        }
    }

    pub fn create_entry ( key: u64, move_: u16, winned: i32) -> Self {
        let w_white: i32;
        let w_black: i32;

        if winned < 0 {
            w_black = winned;
            w_white = 0;
        }
        else {
            w_black = 0;
            w_white = winned;
        }
        Sentry {
            key    : key,
            move_  : move_,
            weight : 0,
            learn  : 0,
            win_white: w_white,
            win_black: w_black,
            total_games: 1,
        }
    }
}
 
pub struct Sbook {
    pub btree: BTreeMap<u64, Vec<Sentry>>,
} 
 
impl Sbook {
    pub fn new() -> Self {
        Sbook {
            btree: BTreeMap::new(),
        }
    }


    // book_clear()

    pub fn book_clear(&mut self) {

        self.btree = BTreeMap::new();
    }

    pub fn insert_move (&mut self, hash_key: u64,
            mov: u16, resul: i32) {
        
        let found = self.btree.get_mut(&hash_key);  // -> Option<&mut V>
        match found {
            Some(elems) => {
                modify_node (elems, hash_key, mov, resul);
            },
            None => { 
                self.append_hash(hash_key, mov, resul);
            },
        };
        /*
        let length_vec = self.btree[&hash_key].len();
        if  length_vec > 10 {
            println!("vector length {} - {}", length_vec, self.btree[&hash_key][1].win_black);
        }
        */
    }

    pub fn append_hash (&mut self, hash_key: u64,
            mov: u16, resul: i32) {
        
        let entry = Sentry::create_entry(hash_key, mov, resul);
        
        // create the new entry in BTree
        let mut vector: Vec<Sentry> = Vec::new();
        vector.push(entry);

        self.btree.insert(hash_key, vector);
    }


    pub fn do_calculations (&mut self) {
        for (_key, value) in self.btree.iter_mut() {
            make_weight(value);
        }
    }
    
}



// External functions

pub fn modify_node (elems: &mut Vec<Sentry>, hash_key: u64, mov: u16, resul: i32) {

    // find hash in vector
    let mut found: bool = false;

    for i in 0..elems.len() {
        if elems[i].key == hash_key && elems[i].move_ == mov {
            if resul < 0 {
                elems[i].win_black += 1;
            }
            else {
                elems[i].win_white += 1;
            }
            elems[i].total_games += 1;
            found = true;
            break;
        } 
    }

    if !found {
        let new_elem = Sentry::create_entry(hash_key, mov, resul);
        elems.push(new_elem);
    }

}


fn make_weight (elems: &mut Vec<Sentry>) {
    for i in 0..elems.len() {
        // 2*(win_white)+(draws);  draws = total_games - win_white - win_black)
        let draws: i32 = elems[i].total_games -
                            elems[i].win_white -
                            elems[i].win_black;
        let resul: i32 = 2 * elems[i].win_white + draws;
        elems[i].weight = resul as u16;
    }
}
