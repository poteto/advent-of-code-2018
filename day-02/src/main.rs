use std::io;
use std::fs;
use std::collections::HashMap;

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

fn main() -> Result<(), Box<std::error::Error>> {
  let input_path = String::from("day-02/INPUT");
  let input = fs::read_to_string(&input_path)?;

  let p1 = part_one(&input)?;
  println!("Part 1: Result is {}", p1);

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
}
