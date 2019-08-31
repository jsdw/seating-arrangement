use crate::tables::Seats;
use itertools::Itertools;

#[derive(Clone)]
pub struct Cost<T,F> {
    cost_fn: F,
    _t: std::marker::PhantomData<T>
}

impl <T: Clone,F: Fn(&T,&T) -> Option<isize>> Cost<T,F> {

    pub fn new(cost_fn: F) -> Cost<T,F> {
        Cost {
            cost_fn,
            _t: std::marker::PhantomData
        }
    }

    /// Get the total cost given some seats
    pub fn total_cost(&self, seats: &Seats<'_,T>) -> isize {
        let mut cost = 0;
        for indexes in seats.indexes_on_each_table() {
            let combinations = indexes
                .map(|idx| &seats[idx])
                .flat_map(|i| i.as_ref())
                .tuple_combinations();
            for (a,b) in combinations {
                cost += (self.cost_fn)(a, b).unwrap_or(0)
            }
        }
        cost
    }

    /// Get the cost of the individual at the index provided
    pub fn individual_cost(&self, seats: &Seats<'_,T>, index: usize) -> isize {
        let mut cost = 0;
        for idx in seats.indexes_on_table_with_index(index) {
            if idx == index {
                continue
            }
            if let (Some(a), Some(b)) = (&seats[index], &seats[idx]) {
                cost += (self.cost_fn)(a, b).unwrap_or(0)
            }
        }
        cost
    }

}
