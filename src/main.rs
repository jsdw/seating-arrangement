#[macro_use] mod errors;

use structopt::StructOpt;
use std::path::{ Path, PathBuf };
use errors::Error;

#[derive(StructOpt, Debug)]
#[structopt(name = "seating")]
struct Opt {

    #[structopt(short="s", long="scores", parse(from_os_str))]
    scores: PathBuf,

    #[structopt(short="t", long="tables")]
    tables: Vec<TableSpec>

}

#[derive(Debug)]
struct TableSpec {
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

#[derive(Debug)]
struct Score {
    value: isize,
    person1: String,
    person2: String
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);

    let scores = load_scores(&opt.scores);
    println!("{:?}", scores)

    // Load the Scores CSV
    // Use a GA where each unit is a person, or an empty space
    // Fixed width, so always same length.
    // For mutation step(s), swap two people.
    // To calculate score:
    //   split GA 'string' into tables
    //   for each person on a table, sum the score of that person against each other person (0 if no score).
    //   then, sum these values together to get a score for the whole table
    //   add all table scores together for a grand total.
    // Score to beat: -13650
    // Runtime: ~20secs on old laptop
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
