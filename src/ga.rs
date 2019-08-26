use rand::seq::{ SliceRandom, index };
use rand::Rng;
use std::mem;
use std::cmp::Ordering;

pub struct Stepper<T, F> {
    population: Vec<(Vec<T>,isize)>,
    fitness_function: F,
    mutation_chance: f64
}

pub struct Opts<'a,T,F> {
    pub initial_value: &'a [T],
    pub fitness_function: F,
    pub population_size: usize,
    pub mutation_chance: f64
}

impl <T: std::fmt::Debug + Clone, F: FnMut(&[T]) -> isize> Stepper<T, F> {

    /// Instantiate a new GA from the options given
    pub fn new(opts: Opts<T,F>) -> Stepper<T,F> {
        let mut rng = rand::thread_rng();
        let mut fitness_function = opts.fitness_function;
        let mut population = Vec::with_capacity(opts.population_size);
        for _ in 0..opts.population_size {
            let mut entry = opts.initial_value.to_owned();
            entry.shuffle(&mut rng);
            let score = fitness_function(&entry);
            population.push((entry, score));
        }

        let mut ga = Stepper {
            population,
            fitness_function,
            mutation_chance: opts.mutation_chance
        };

        // Prepare the population for the first iteration:
        ga.sort_population_by_best_first();
        ga
    }

    /// Get the best entry and score out:
    pub fn best_entry(&self) -> (&[T], isize) {
        let best = &self.population[0];
        (&best.0, best.1)
    }

    pub fn step(&mut self) {
        self.breed_next_population();
        self.sort_population_by_best_first();
    }

    fn sort_population_by_best_first(&mut self) {
        self.population.sort_by_key(|(_,s)| *s);
    }

    fn breed_next_population(&mut self) {
        let mut rng = rand::thread_rng();
        let population_size = self.population.len();
        let mut population = mem::replace(&mut self.population, Vec::with_capacity(population_size));

        // A Roulette wheel where higher scoring entries have more chance of being picked:
        let worst_score = population.last().unwrap().1;
        let roulette = Roulette::new(population.iter().map(|(_,s)| *s), worst_score);

        // Keep the best entry:
        self.population.push(population[0].clone());

        // Pick entries to crossover (and occasionally mutate)
        // until we've filles up the new population:
        while self.population.len() < population_size {

            let a_idx = roulette.choose_idx();
            let mut new_a = population[a_idx].0.clone(); // crossover(&population[a_idx].0);

            while rng.gen_range(0f64,1f64) < self.mutation_chance {
                mutate(&mut new_a)
            }

            self.population.push(with_fitness(new_a, &mut self.fitness_function));
        }
    }

}

fn with_fitness<T: Clone, F: FnMut(&[T]) -> isize>(val: Vec<T>, mut fitness_function: F) -> (Vec<T>, isize) {
    let fitness = fitness_function(&val);
    (val, fitness)
}

fn crossover<T: Clone>(a: &[T]) -> Vec<T> {
    let len = a.len();
    let mut rng = rand::thread_rng();

    // Cut a chunk out of the seating arrangement:
    let (cut1, cut2) = {
        let a = rng.gen_range(0, len);
        let b = rng.gen_range(0, len);
        if a < b { (a,b) } else { (b,a) }
    };

    let mut sections = [&a[..cut1], &a[cut1..cut2], &a[cut2..]];
    sections.shuffle(&mut rng);

    sections.into_iter().map(|&s| s).flatten().cloned().collect()
}

fn mutate<T>(a: &mut [T]) {
    let mut rng = rand::thread_rng();
    let idxs = index::sample(&mut rng, a.len(), 2);

    a.swap(idxs.index(0), idxs.index(1));
}

/// A Roulette wheel which is more likely to pick indexes that have higher scores
struct Roulette {
    sum: usize,
    ranges: Vec<(usize,usize)>
}

impl Roulette {
    fn new(scores: impl Iterator<Item=isize>, worst: isize) -> Roulette {
        let mut sum = 0;
        let mut ranges = vec![];
        for score in scores {
            let top = sum + (worst - score) as usize;
            ranges.push((sum, top));
            sum = top;
        }
        Roulette {
            sum,
            ranges
        }
    }

    fn choose_idx(&self) -> usize {
        let mut rng = rand::thread_rng();

        if 0 == self.sum {
            // Everything is equally good/bad, so just
            // pick something entirely at random:
            return rng.gen_range(0, self.ranges.len());
        }

        let val: usize = rng.gen_range(0, self.sum);
        let idx = self.ranges.binary_search_by(|&(low,high)| {
            if val < low {
                Ordering::Greater
            } else if val >= high {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        }).expect("index should always be findable in range");
        idx
    }
}