use std::io;
use std::fs;

type Frequency = i32;

fn part_one(init_freq: Frequency, input: &str) -> io::Result<()> {
    let res = fs::read_to_string(&input)
        .unwrap()
        .lines()
        .filter_map(|l| l.parse::<Frequency>().ok())
        .fold(init_freq, |acc, curr| acc + curr);
    println!("Part 1: Result is {}", res);
    Ok(())
}

fn main() -> io::Result<()> {
    let input = String::from("INPUT");
    part_one(0, &input)?;
    Ok(())
}