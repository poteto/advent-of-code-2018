use std::io;
use std::fs;
use std::collections::HashSet;

type Frequency = i32;
type SeenFrequencies = HashSet<Frequency>;

pub fn part_one(init_freq: Frequency, input: &str) -> io::Result<Frequency> {
    let res = input
        .lines()
        .filter_map(|l| l.parse::<Frequency>().ok())
        .fold(init_freq, |acc, curr| acc + curr);
    Ok(res)
}

pub fn part_two(init_freq: Frequency, input: &str) -> io::Result<Frequency> {
    let mut res = init_freq;
    let mut seen: SeenFrequencies = HashSet::new();
    seen.insert(res);
    let iter = input
        .lines()
        .filter_map(|l| l.parse::<Frequency>().ok())
        .cycle();
    for change in iter {
        res += change;
        if seen.contains(&res) {
            break;
        }
        seen.insert(res);
    }
    Ok(res)
}

fn main() -> io::Result<()> {
    let input_path = String::from("INPUT");
    let input = fs::read_to_string(&input_path)?;

    let p1 = part_one(0, &input)?;
    println!("Part 1: Result is {}", p1);

    let p2 = part_two(0, &input)?;
    println!("Part 2: Result is {}", p2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input1 = "+1\n+1\n+1";
        let input2 = "+1\n+1\n-2";
        let input3 = "-1\n-2\n-3";

        assert_eq!(part_one(0, &input1).unwrap(), 3);
        assert_eq!(part_one(0, &input2).unwrap(), 0);
        assert_eq!(part_one(0, &input3).unwrap(), -6);
    }

    #[test]
    fn test_part_two() {
        let input1 = "+1\n-1";
        let input2 = "+3\n+3\n+4\n-2\n-4";
        let input3 = "-6\n+3\n+8\n+5\n-6";
        let input4 = "+7\n+7\n-2\n-7\n-4";

        assert_eq!(part_two(0, &input1).unwrap(), 0);
        assert_eq!(part_two(0, &input2).unwrap(), 10);
        assert_eq!(part_two(0, &input3).unwrap(), 5);
        assert_eq!(part_two(0, &input4).unwrap(), 14);
    }
}