use std::cmp;
use std::collections::HashSet;
use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use std::io::{BufWriter, Seek, SeekFrom, Write};

use rayon::prelude::*;
use rand::prelude::*;

pub fn generate_combinations(digit_count: u32, insert_at_random: bool) -> std::io::Result<()> {
  let path = PathBuf::from("combinations");
  if path.is_dir() {
    fs::remove_dir_all(&path)?;
  }

  fs::create_dir(&path)?;

  let n = 64_i64.pow(digit_count);
  let group_size = 140_000_000 / (digit_count + 1) as i64; // Set file size to 140mb when taking into account a one byte separator between each token
  let offset = if digit_count - 1 == 0 {
    0
  }
  else {
    64_i64.pow(digit_count - 1)
  };

  let number_of_files = (n - offset) / group_size + 1;
  let number_of_digits_in_number_of_files = match number_of_files {
    1..=9     => 1,
    10..=99   => 2,
    100..=999 => 3,
    _ => panic!("Attempting to create 1000 files or more. Probably not intended.")
  };

  (0..number_of_files)
    .into_iter()
    .map(|i| {
        let start: i64 = (i * group_size) + offset;
        let end: i64 = cmp::min((i + 1) * group_size + offset, n);
        start..end
    })
    .collect::<Vec<_>>()
    .par_iter_mut()
    .enumerate()
    .try_for_each(|(index, range)| {
      let mut filename = path.clone();
      // 0 pad numeric filenames
      let identifier = format!("{:0width$}", index, width = number_of_digits_in_number_of_files);
      filename.push(identifier);
      // TODO: Get a set from database with any elements in given range
      let set: HashSet<String> = HashSet::new();
      let f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&filename)?;
      {
        let mut combination_count = 0;
        let mut writer = BufWriter::new(f);
        let mut rng = rand::thread_rng();
        for i in range {
          let mut digit = to_base64_string(i);
          let s = match String::from_utf8(digit.clone()) {
            Ok(s) => s,
            Err(err) => panic!("Issue when trying to convert a {:?} to a String: {}", digit, err)
          };

          if set.contains(&s) {
            continue;
          }

          digit.push(b'\\');

          if insert_at_random && combination_count > 0 {
            let position = rng.gen_range(0..=combination_count) * (digit_count + 1);
            writer.seek(SeekFrom::Start(position as u64))?;
          }

          writer.write(&digit)?;
          combination_count += 1;
        }
      }

      Ok(())
  })
}

// TODO: Get current digit count from database
pub fn get_digit_count() -> u32 {
  return 5;
}

const BASE64_CHARS: &'static [u8; 64] = &[b'0',b'1',b'2',b'3',b'4',b'5',b'6',b'7',b'8',b'9',b'a',b'b',b'c',b'd',b'e',b'f',b'g',b'h',b'i',b'j',b'k',b'l',b'm',b'n',b'o',b'p',b'q',b'r',b's',b't',b'u',b'v',b'w',b'x',b'y',b'z',b'A',b'B',b'C',b'D',b'E',b'F',b'G',b'H',b'I',b'J',b'K',b'L',b'M',b'N',b'O',b'P',b'Q',b'R',b'S',b'T',b'U',b'V',b'W',b'X',b'Y',b'Z',b'+',b'-'];
fn to_base64_string(mut n: i64) -> Vec<u8> {
  let mut result = Vec::<u8>::new();
  // do while like behavior
  while {
    let remainder = (n % 64) as usize;
    n /= 64;
    result.push(BASE64_CHARS[remainder]);

    // If this evaluates to false, the loop terminates
    n > 0 
  } {}

  result.reverse();
  result
}
