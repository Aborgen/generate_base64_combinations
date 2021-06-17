use std::time::Instant;

mod base;
mod database;

use base::CombinationResult;
use database::DatabaseActions;

// 4 digits takes ~2 seconds  (non-random) 79mb
// 5 digits takes ~50 seconds (non-random) 6gb
// 6 digits would take up ~474 gb. Probably don't try that.

fn main() {
  let result = || -> CombinationResult<()> {
    let db = DatabaseActions::new("localhost", 6379)?;
    let digit_count = base::get_digit_count(&db)?;
    let insert_at_random = false;
    let start = Instant::now();
    base::generate_combinations(digit_count, insert_at_random, db)?;
    println!("Time elapsed: {:?}", start.elapsed());
    Ok(())
  };

  match result() {
    Ok(_) => (),
    Err(err) => println!("{}", err)
  };
}

