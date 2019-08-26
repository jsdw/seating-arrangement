use rand::seq::{ SliceRandom, index };
use rand::Rng;

pub struct Optimiser<T, F> {
    population: Vec<Vec<T>>,
    fitness_function: F
}

pub struct Opts<'a,T,F> {
    pub initial_value: &'a [T],
    pub fitness_function: F,
    pub population_size: usize,
}

impl <T: std::fmt::Debug + Clone, F: FnMut(&[T]) -> isize> Optimiser<T, F> {

    /// Instantiate a new optimiser from the options given
    pub fn new(opts: Opts<T,F>) -> Optimiser<T,F> {
        let mut rng = rand::thread_rng();
        let fitness_function = opts.fitness_function;
        let mut population = Vec::with_capacity(opts.population_size);
        for _ in 0..opts.population_size {
            let mut entry = opts.initial_value.to_owned();
            entry.shuffle(&mut rng);
            population.push(entry);
        }

        let mut optimiser = Optimiser {
            population,
            fitness_function
        };

        // Prepare the population for the first iteration:
        optimiser.sort_population_by_best_first();
        optimiser
    }

    /// Get the best entry and score out:
    pub fn best_entry(&self) -> &[T] {
        &self.population[0]
    }

    /// Perform one step of the algorithm:
    pub fn step(&mut self) {
        self.breed_next_population();
        self.sort_population_by_best_first();
    }

    fn breed_next_population(&mut self) {
        let mut rng = rand::thread_rng();
        let len = self.population.len();
        let mut new_population = Vec::with_capacity(len);

        // Always keep the winner:
        new_population.push(self.population[0].clone());

        for _ in 1..len {
            // Random index skewed towards starting entries:
            let idx = (rng.gen_range(0.0f64,1.0).powf(2.0) * len as f64).floor() as usize;
            let mut entry = self.population.get(idx).unwrap().clone();

            mutate(&mut entry);
            new_population.push(entry);
        }
        self.population = new_population;
    }

    fn sort_population_by_best_first(&mut self) {
        let fitness_function = &mut self.fitness_function;
        self.population.sort_by_cached_key(|seats| fitness_function(seats));
    }

}

fn mutate<T>(a: &mut [T]) {
    let mut rng = rand::thread_rng();
    let idxs = index::sample(&mut rng, a.len(), 2);
    a.swap(idxs.index(0), idxs.index(1));
}
