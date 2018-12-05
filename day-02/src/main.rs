extern crate itertools;

use std::io;
use std::fs;
use std::collections::HashMap;
use itertools::Itertools;

type Checksum = i32;
type Seen = HashMap<char, i32>;

pub fn part_one(input: &str) -> io::Result<Checksum> {
  let mut doubles = 0;
  let mut triples = 0;
  let iter = input.lines().map(|l| l.chars());
  for chars in iter {
    let mut seen: Seen = HashMap::new();
    for c in chars { *seen.entry(c).or_insert(0) += 1; }
    if seen.iter().any(|s| s.1 == &2) { doubles += 1 };
    if seen.iter().any(|s| s.1 == &3) { triples += 1 };
  }
  Ok(doubles * triples)
}

pub fn part_two(input: &str) -> Option<String> {
  let pairs = input.lines().combinations(2);
  for pair in pairs {
    let mut idx = 0;
    let mut diff = 0;
    let chars = pair[0].chars().zip(pair[1].chars()).enumerate();
    for (i, (a, b)) in chars {
      if a != b { diff += 1; idx = i; }
      if diff > 1 { break; }
    }
    if diff == 1 {
      let res = pair[0][0..idx].to_owned() + &pair[0][idx+1..];
      return Some(res);
    }
  }
  None
}

fn main() -> Result<(), Box<std::error::Error>> {
  let input_path = String::from("day-02/INPUT");
  let input = fs::read_to_string(&input_path)?;

  let p1 = part_one(&input)?;
  println!("Part 1: Result is {}", p1);

  if let Some(p2) = part_two(&input) {
    println!("Part 2: Result is {}", p2);
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_part_one() {
    let input = "abcdef\nbababc\nabbcde\nabcccd\naabcdd\nabcdee\nababab";

    assert_eq!(part_one(&input).unwrap(), 12);
  }

  #[test]
  fn test_part_two() {
    let input = "abcde\nfghij\nklmno\npqrst\nfguij\naxcye\nwvxyz";

    assert_eq!(part_two(&input).unwrap(), "fgij");
  }
}
