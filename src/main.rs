#[macro_use] mod errors;
mod ga;
mod scores;
mod person;
mod tables;

use itertools::Itertools;
use structopt::StructOpt;
use std::path::{ Path, PathBuf };
use errors::Error;
use scores::Scores;
use person::{ NameToId, Id };
use tables::{ TableSpec, Tables };

#[derive(StructOpt, Debug)]
#[structopt(name = "seating")]
struct Opt {
    /// A path to a CSV file containing pairs of individuals and a score
    /// for how much these people should be sat together.
    #[structopt(short="s", long="scores", parse(from_os_str))]
    score_list_file: PathBuf,
    /// A description of the available tables to seat people on.
    #[structopt(short="t", long="tables")]
    table_specs: Vec<TableSpec>
}

fn main() -> Result<(),Error> {
    let opt = Opt::from_args();
    let score_list = load_scores(&opt.score_list_file)?;
    let tables: Tables = opt.table_specs.into();

    // build map of name to Id:
    let mut name_to_id = NameToId::new();
    for score in &score_list {
        name_to_id.add_person(score.person1.to_owned());
        name_to_id.add_person(score.person2.to_owned());
    }

    // build map of pair-of-names to score:
    let mut scores = Scores::new();
    for score in &score_list {
        let person1_id = name_to_id.get_id(&score.person1).unwrap();
        let person2_id = name_to_id.get_id(&score.person2).unwrap();
        scores.add_score(person1_id, person2_id, score.value);
    }

    // Get number of total seats:
    let number_of_seats = tables.num_seats();

    // Build starting seat arrangement:
    let mut seats: Vec<Option<Id>> = name_to_id.iter_ids().map(|id| Some(id)).collect();
    for _ in seats.len()..number_of_seats {
        seats.push(None)
    }

    // Our cost function (lower output is better):
    let cost_fn = |seats: &[Option<Id>]| -> isize {
        let mut cost = 0;
        for (_,ids) in tables.chunks_of(seats) {
            for (a,b) in ids.iter().filter_map(|&id| id).tuple_combinations() {
                cost += scores.get_score(a, b).unwrap_or(0)
            }
        }
        cost
    };

    // Train a GA using the initial seat list, and the cost_fn above.
    let mut genetic_algorithm = ga::Stepper::new(ga::Opts{
        population_size: 3,
        mutation_chance: 0.6,
        fitness_function: cost_fn,
        initial_value: &seats
    });
    for it in 0..1000000 {
        genetic_algorithm.step();
        println!("{} Best: {:?}", it+1, genetic_algorithm.best_entry().1);
    }

    let (seats, _) = genetic_algorithm.best_entry();

    for (table_size, ids) in tables.chunks_of(seats) {
        println!("Table Size {}:", table_size);
        for id in ids {
            if let Some(id) = id {
                let name = name_to_id.get_name(*id).expect("Id should have a corresponding name");
                println!("- {}", name);
            }
        }
        println!();
    }

    Ok(())
}

fn load_scores(file_path: &Path) -> Result<Vec<Score>,Error> {
    csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)
        .map_err(|e| err!("Could not load scores: {}", e))?
        .into_records()
        .enumerate()
        .map(|(idx,record)| {
            let record = record.map_err(|e| {
                err!("Score entry {} is invalid: {}", idx+1, e)
            })?;
            if record.len() != 3 {
                return Err(err!("Score entry {} has {} columns but was expecting 3", idx+1, record.len()))
            }
            Ok(Score {
                value: record[0]
                    .parse()
                    .map_err(|_| err!("Score entry {}'s score is not a number, is '{}'", idx+1, &record[0]))?,
                person1: record[1].to_string(),
                person2: record[2].to_string()
            })
        })
        .collect()
}

#[derive(Debug)]
struct Score {
    value: isize,
    person1: String,
    person2: String
}