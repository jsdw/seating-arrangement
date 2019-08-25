use crate::errors::Error;

/// A set of tables:
#[derive(Debug,PartialEq,Eq,Clone)]
pub struct Tables {
    seats: Vec<usize>
}

impl Tables {
    pub fn num_seats(&self) -> usize {
        self.seats.iter().map(|&n| n).sum()
    }
    /// Slice the thing passed in into slices on seat length for each table we know about,
    /// returning the table size and the slice of items for the table. If there are not enough items,
    /// the final slice returned will be smaller than the table size.
    pub fn chunks_of<'t, T>(&'t self, item: &'t [T]) -> impl Iterator<Item=(usize,&'t [T])> + 't {

        let mut seats_idx = 0;
        let mut start_idx = 0;

        std::iter::from_fn(move || {
            let seat_count = *self.seats.get(seats_idx)?;

            // If not enough people to fill the seats, the end list will be smaller:
            let mut end_idx = start_idx + seat_count;
            if end_idx >= item.len() { end_idx = item.len() }

            let slice = item.get(start_idx..end_idx)?;
            seats_idx += 1;
            start_idx += seat_count;
            Some((seat_count, slice))
        }).fuse()
    }
}

impl std::convert::From<Vec<TableSpec>> for Tables {
    fn from(spec: Vec<TableSpec>) -> Tables {
        let mut seats = Vec::new();
        for ts in spec {
            for _ in 0..ts.quantity {
                seats.push(ts.seats);
            }
        }
        Tables { seats }
    }
}

/// A specification of the tables we have, that can be derived from strings:
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