use rand::Rng;

use crate::tables::Seats;
use crate::ids::Id;
use crate::cost::Cost;

pub struct Search<'t,F> {
    cost: Cost<Id,F>,
    current: Seats<'t, Id>,
    current_cost: isize,
    temperature: f64
}

impl <'t,F> Search<'t,F> where F: Fn(&Id,&Id) -> Option<isize> {

    pub fn new(seats: Seats<'t, Id>, cost_fn: F) -> Search<'t,F> {
        let cost = Cost::new(cost_fn);
        let current_cost = cost.total_cost(&seats);
        Search {
            cost,
            current: seats,
            current_cost,
            temperature: 0.0
        }
    }

    pub fn step(&mut self) {
        let mut rng = rand::thread_rng();

        let n = rng.gen_range(1, 5);
        let m = Moves::random(&self.current, n);
        let c = m.cost_of_moves(&mut self.current, &self.cost);

        // Apply move if it's an improvement, or if the temperature
        // is warm enough that it's randomly allowed:
        if c < self.cost() || rng.gen_range(0.0,1.0) < self.temperature {
            m.apply(&mut self.current);
            self.current_cost = c;
            // Each time we apply a move, knock the temperature right down:
            self.temperature -= 0.2;
        } else {
            // Each time we can't apply the move, increase chance that we
            // apply a bad move a little:
            self.temperature += 0.0000000001;
        }
        // Temperature can't get below 0:
        self.temperature = self.temperature.max(0.0);
    }

    pub fn best(&self) -> &Seats<'t,Id> {
        &self.current
    }

    pub fn cost(&self) -> isize {
        self.current_cost
    }

}

struct Moves {
    moves: Vec<Move>
}

impl Moves {
    /// Create a sequence of random moves
    pub fn random<'t>(seats: &Seats<'t,Id>, count: usize) -> Moves {
        let moves = (0..count).map(|_| Move::random(seats)).collect();
        Moves { moves }
    }
    /// Simulate the move and work out the net cost change of doing so
    pub fn cost_of_moves<'t,F>(&self, seats: &mut Seats<'t,Id>, cost: &Cost<Id,F>) -> isize where F: Fn(&Id,&Id) -> Option<isize> {
        self.apply(seats);
        let after = cost.total_cost(seats);
        self.unapply(seats);
        after
    }
    /// Apply the move
    pub fn apply<'t>(&self, seats: &mut Seats<'t,Id>) {
        for m in &self.moves {
            m.apply(seats);
        }
    }
    /// Unapply some applied moves
    pub fn unapply<'t>(&self, seats: &mut Seats<'t,Id>) {
        for m in self.moves.iter().rev() {
            m.apply(seats);
        }
    }
}

struct Move {
    a: usize,
    b: usize
}

impl Move {
    /// Create a random move swapping two people between tables
    pub fn random<'t>(seats: &Seats<'t,Id>) -> Move {
        let mut rng = rand::thread_rng();
        let a = rng.gen_range(0, seats.len());
        let a_table = seats.indexes_on_table_with_index(a);
        let b = loop {
            let possible_b = rng.gen_range(0, seats.len());
            if a_table.contains(&possible_b) {
                continue
            } else {
                break possible_b
            }
        };
        Move { a, b }
    }
    /// Simulate the move and work out the net cost change of doing so
    pub fn cost_of_move<'t,F>(&self, seats: &mut Seats<'t,Id>, cost: &Cost<Id,F>) -> isize where F: Fn(&Id,&Id) -> Option<isize> {
        self.apply(seats);
        let after = cost.individual_cost(seats, self.a) + cost.individual_cost(seats, self.b);
        self.apply(seats);
        after
    }
    /// Apply the move
    pub fn apply<'t>(&self, seats: &mut Seats<'t,Id>) {
        seats.swap(self.a, self.b);
    }
}
