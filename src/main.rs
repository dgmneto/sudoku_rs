mod grid;
mod solver;

use quicli::prelude::*;
use structopt::StructOpt;

use csv::Reader;

/// Solve Sudoku grids
#[derive(Debug, StructOpt)]
struct Cli {
    /// CSV column to use as puzzle input
    #[structopt(long = "puzzle-column", short = "p", default_value = "puzzle")]
    puzzle_column: String,

    /// CSV input file with puzzles and extra columns that won't be parsed
    file: String,

    // Quick and easy logging setup you get for free with quicli
    #[structopt(flatten)]
    verbosity: Verbosity,
}

fn main() -> CliResult {
    let args = Cli::from_args();
    args.verbosity.setup_env_logger("sudoku_rs")?;

    let mut reader = Reader::from_path(&args.file)?;
    let headers = reader.headers()?;
    let puzzle_col = match headers
        .into_iter()
        .position(|col| col == args.puzzle_column)
    {
        Some(col) => col,
        None => {
            return {
                error!(
                    "Column {:?} is not present in header {:?}.",
                    &args.puzzle_column,
                    headers.as_slice()
                );
                Ok(()) // TODO return Err
            };
        }
    };

    for record_result in reader.records() {
        let record = record_result?;
        let g = grid::from_line(&record[puzzle_col]);
        match solver::solve(g, 0, 0) {
            Ok(g) => println!("{},{}", &record[puzzle_col], g),
            Err(g) => {
                println!("Didn't find solution {} {:?}", &record[puzzle_col], g);
                return Ok(());
            }
        }
    }

    Ok(())
}
