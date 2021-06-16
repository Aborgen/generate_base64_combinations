use std::time::Instant;

mod lib;

// 4 digits takes ~2 seconds  (non-random) 79mb
// 5 digits takes ~50 seconds (non-random) 6gb
// 6 digits would take up ~474 gb. Probably don't try that.

fn main() {
  let digit_count = lib::get_digit_count();
  let insert_at_random = false;
  let start = Instant::now();
  match lib::generate_combinations(digit_count, insert_at_random) {
    Ok(_) => (),
    Err(err) => println!("{}", err)
  };

  println!("Time elapsed: {:?}", start.elapsed());
}

