use std::num::NonZeroU32;
use std::collections::HashMap;

#[repr(transparent)]
#[derive(Debug,Eq,PartialEq,Clone,Copy,Hash,PartialOrd,Ord)]
pub struct Id(NonZeroU32);

/// A mapping from Id to name and vice versa
pub struct NameToId {
    next_id: u32,
    to_id: HashMap<String,Id>,
    from_id: HashMap<Id,String>
}

impl NameToId {
    pub fn new() -> NameToId {
        NameToId {
            next_id: 1,
            to_id: HashMap::new(),
            from_id: HashMap::new()
        }
    }
    pub fn add_person(&mut self, person: String) -> Id {
        if let Some(id) = self.to_id.get(&person) {
            *id
        } else {
            let id = Id(NonZeroU32::new(self.next_id).unwrap());
            self.next_id += 1;
            self.to_id.insert(person.clone(), id);
            self.from_id.insert(id, person);
            id
        }
    }
    pub fn get_id(&self, person: &str) -> Option<Id> {
        self.to_id.get(person).map(|&id| id)
    }
    pub fn get_name(&self, id: Id) -> Option<&str> {
        self.from_id.get(&id).map(|name| &**name)
    }
    pub fn iter_ids(&self) -> impl Iterator<Item=Id> + '_ {
        self.from_id.keys().map(|&k| k)
    }
}


/// A mapping from a pair of Ids to a score
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