use std::collections::HashMap;
use crate::person::Id;

/// A Map of individual scores
#[derive(Debug,Clone)]
pub struct Scores {
    map: HashMap<IdPair, isize>
}

impl Scores {
    pub fn new() -> Scores {
        Scores {
            map: HashMap::new()
        }
    }
    pub fn add_score(&mut self, a: Id, b: Id, score: isize) {
        self.map.insert(IdPair::new(a,b), score);
    }
    pub fn get_score(&self, a: Id, b: Id) -> Option<isize> {
        self.map.get(&IdPair::new(a,b)).map(|&score| score)
    }
}


/// A pair of IDs, to ensure internal ordering
#[derive(Eq,PartialEq,Hash,Debug,Copy,Clone)]
struct IdPair(Id,Id);

impl IdPair {
    fn new(a: Id, b: Id) -> IdPair {
        if a > b { IdPair(b,a) }
        else { IdPair(a,b) }
    }
}