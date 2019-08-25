use std::num::NonZeroU32;
use std::collections::HashMap;

#[repr(transparent)]
#[derive(Debug,Eq,PartialEq,Clone,Copy,Hash,PartialOrd,Ord)]
pub struct Id(NonZeroU32);

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