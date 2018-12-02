use std::fs;
use std::io::Cursor;
use std::io::prelude::*;

fn print_report(curr: i32, change: String, next: i32) {
    println!(
        "Current frequency {curr}, change of {change}; resulting frequency {next}",
        curr = curr,
        change = change,
        next = next
    );
}

fn cursor_from_file(path: &'static str) -> Cursor<String> {
    let contents = fs::read_to_string(path).unwrap();
    return Cursor::new(contents);
}

fn main() -> std::io::Result<()> {
    let mut curr_freq: i32 = 0;
    let lines_iter = cursor_from_file("INPUT").lines().map(|l| l.unwrap());
    for l in lines_iter {
        let change = l.parse::<i32>().unwrap();
        let next_freq = curr_freq + change;
        print_report(curr_freq, l, next_freq);
        curr_freq = next_freq;
    }
    println!("Result is {}", curr_freq);
    Ok(())
}