use std::env;
use std::fs;
use std::io;
use std::process;

struct Counts {
    lines: usize,
    words: usize,
    bytes: usize,
}

fn count_file(path: &str) -> Result<Counts, io::Error> {
    let text = fs::read_to_string(path)?;
    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let bytes = text.len();
    Ok(Counts { lines, words, bytes })
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: word_line_counter [-l] [-w] [-c] <file>");
        process::exit(1);
