// Error handling:
#[macro_use] mod errors;
// Functionality around Ids and scores between pairs of Ids:
mod ids;
// Functionality around table and seat arrangements:
mod tables;
/// Working out the cost of some seating arrangement:
mod cost;
// Algorithm to find the best layout:
mod search;

use structopt::StructOpt;
use std::path::{ Path, PathBuf };
use errors::Error;
use ids::{ NameToId, Scores };
use tables::{ TableSpec, Tables };
use search::Search;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(StructOpt, Debug)]
#[structopt(name = "seating")]
struct Opt {
    /// A path to a CSV file containing pairs of individuals and a score
    /// for how much these people should be sat together.
    #[structopt(short="s", long="scores", parse(from_os_str))]
    score_list_file: PathBuf,
    /// A description of the available tables to seat people on.
    #[structopt(short="t", long="tables")]
    table_specs: Vec<TableSpec>,
    /// How many iterations to perform; more iterations leads to better
    /// results, but also takes longer to complete.
    #[structopt(short="i", long="iterations")]
    iterations: Option<usize>
}

fn main() -> Result<(),Error> {

    let opt = Opt::from_args();
    let score_list = load_scores(&opt.score_list_file)?;
    let tables: Tables = opt.table_specs.into();
    let iterations = opt.iterations.unwrap_or(std::usize::MAX);

    // Handle CTRL+C:
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

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

    // Put the Ids we have onto table seats:
    let seats = tables.create_seats_from(name_to_id.iter_ids())?;

    // Prepare to search the seating arrangement:
    let mut search = Search::new(seats, |a, b| scores.get_score(*a, *b));

    for i in 0..iterations {
        search.step();
        if i % 1000 == 0 {
            println!("{i}: {c}", i=i, c=search.cost());
            if !running.load(Ordering::Relaxed) {
                break
            }
        }
    }

    let final_cost = search.cost();
    let final_seats = search.best();

    for idxs in final_seats.indexes_on_each_table() {
        println!("Table (size {}):", idxs.end - idxs.start);
        for idx in idxs {
            if let Some(id) = final_seats[idx] {
                let name = name_to_id.get_name(id).expect("Id should have a corresponding name");
                println!("- {}", name);
            }
        }
        println!();
    }
    println!("Final score: {}", final_cost);

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