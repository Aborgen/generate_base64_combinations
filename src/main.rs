use std::cmp;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

fn main() {
  ThreadPoolBuilder::new().num_threads(7).build_global().unwrap();


  let digit_count = 5;
  let n = 64_i64.pow(digit_count);
  let group_size = 30000000; // 30MB
  let product: i64 = (n as f64 / group_size as f64).ceil() as i64;

  (0..product)
    .into_iter()
    .map(|i| {
        // Offset ensures that only ranges greater than the maximum value of digit_count - 1 are generated
        let offset = if digit_count - 1 == 0 {
          0
        }
        else {
          64_i64.pow(digit_count - 1)
        };

        let start: i64 = (i * group_size) + offset;
        let end: i64 = cmp::min((i + 1) * group_size + offset, n);
        
        start..end
    })
    .collect::<Vec<_>>()
    .par_iter_mut()
    .enumerate()
    .for_each(|(index, range)| {
      let filename = format!("file_{}", index);
      let set: HashSet<String> = HashSet::new();
      let f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&filename).unwrap();
      println!("Range: {:?}, {}", range, filename);
      {
        let mut writer = BufWriter::new(f);
        for i in range {
          let mut digit = to_base64_string(i);
          if !set.contains(&String::from_utf8(digit.clone()).unwrap()) {
            digit.push(b'\\');
            writer.write(&digit).unwrap();
          }
        }
      }
    });
}

fn to_base64_string(mut n: i64) -> Vec<u8> {
  let base64_chars: [u8; 64] = [b'0',b'1',b'2',b'3',b'4',b'5',b'6',b'7',b'8',b'9',b'a',b'b',b'c',b'd',b'e',b'f',b'g',b'h',b'i',b'j',b'k',b'l',b'm',b'n',b'o',b'p',b'q',b'r',b's',b't',b'u',b'v',b'w',b'x',b'y',b'z',b'A',b'B',b'C',b'D',b'E',b'F',b'G',b'H',b'I',b'J',b'K',b'L',b'M',b'N',b'O',b'P',b'Q',b'R',b'S',b'T',b'U',b'V',b'W',b'X',b'Y',b'Z',b'+',b'-'];
  let mut result = Vec::<u8>::new();
  // do while-like behavior
  while {
    let remainder = (n % 64) as usize;
    n /= 64;
    result.push(base64_chars[remainder]);

    // If this evaluates to false, the loop terminates
    n > 0 
  } {}

  result.reverse();
  result
}
