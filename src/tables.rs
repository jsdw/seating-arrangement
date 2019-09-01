use crate::errors::Error;
use std::ops::Range;
use std::cmp::Ordering;

/// A set of tables:
#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Tables {
    seats: Vec<Range<usize>>
}

impl Tables {
    /// How many seats are there in total?
    pub fn seat_count(&self) -> usize {
        self.seats.last().map(|r| r.end).unwrap_or(0)
    }
    /// Hand back an iterator over the range of indexes on each table.
    pub fn table_indexes<'a>(&'a self) -> impl Iterator<Item=Range<usize>> + 'a {
        self.seats.iter().cloned()
    }
    /// What's the range of indexes of the table containing the provided index?
    pub fn table_indexes_containing(&self, index: usize) -> Range<usize> {
        let idx = self.seats.binary_search_by(|r| {
            if r.start > index {
                Ordering::Greater
            } else if r.end <= index {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }).expect("index out of range");
        self.seats[idx].clone()
    }
    /// Put some items into Seats, complaining if there aren't enough seats to fit them all:
    pub fn create_seats_from<'a,T>(&'a self, items: impl IntoIterator<Item=T>) -> Result<Seats<'a,T>,Error> {
        let num_seats = self.seat_count();
        let mut seated_items: Vec<_> = items.into_iter().map(Option::Some).collect();

        if seated_items.len() > num_seats {
            return Err(err!("Not enough seats on the table; need at least {} but have {}", seated_items.len(), num_seats))
        }

        while seated_items.len() < num_seats {
            seated_items.push(None);
        }

        Ok(Seats {
            tables: self,
            items: seated_items
        })
    }
}

/// Convert a TableSpec into Tables:
impl std::convert::From<Vec<TableSpec>> for Tables {
    fn from(spec: Vec<TableSpec>) -> Tables {
        let mut seats = Vec::new();
        let mut last_idx = 0;
        for ts in spec {
            for _ in 0..ts.quantity {
                let new_last_idx = last_idx + ts.seats;
                seats.push(last_idx .. new_last_idx);
                last_idx = new_last_idx;
            }
        }
        Tables { seats }
    }
}

/// A specification of the tables we have that can be derived from strings:
#[derive(Debug,PartialEq,Eq,Clone,Copy)]
pub struct TableSpec {
    seats: usize,
    quantity: usize
}

impl std::str::FromStr for TableSpec {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 'A' is the number of seats on a table:
        if let Ok(n) = s.parse() {
            Ok(TableSpec { seats: n, quantity: 1 })
        }
        // 'AxB' is the number of seats * the number of tables with that many seats:
        else if let Some(idx) = s.find(|c| c == 'x' || c == 'X') {
            let seats_str = &s[0..idx];
            let quantity_str = &s[idx+1..];

            let seats = seats_str.parse().map_err(|_| {
                err!("'{}' is not a valid number of seats", seats_str)
            })?;
            let quantity = quantity_str.parse().map_err(|_| {
                err!("'{}' is not a valid number of tables", quantity_str)
            })?;

            Ok(TableSpec { seats, quantity })
        }
        // None of the above is an error:
        else {
            Err(err!("Table quantity needs to be provided as 'A' or 'AxB' where 'A' is the\
            number of seats and 'B' is the number of tables with that number of seats"))
        }
    }
}

/// Seats containing T's on a table (empty seats included)
#[derive(Debug,Clone)]
pub struct Seats<'tables, T> {
    tables: &'tables Tables,
    items: Vec<Option<T>>
}

impl <'tables, T> Seats<'tables, T> {
    pub fn indexes_on_each_table<'a>(&'a self) -> impl Iterator<Item=Range<usize>> + 'a {
        self.tables.table_indexes()
    }
    pub fn indexes_on_table_with_index<'a>(&'a self, idx: usize) -> Range<usize> {
        self.tables.table_indexes_containing(idx)
    }
}

impl <T> std::ops::Deref for Seats<'_,T> {
    type Target = [Option<T>];
    fn deref(&self) -> &Self::Target {
        &self.items
    }
}
impl <T> std::ops::DerefMut for Seats<'_,T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}