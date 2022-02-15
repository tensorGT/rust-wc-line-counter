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
    }

    let mut show_l = false;
    let mut show_w = false;
    let mut show_c = false;
    let mut file_path: Option<&str> = None;

    for arg in &args[1..] {
        match arg.as_str() {
            "-l" => show_l = true,
            "-w" => show_w = true,
            "-c" => show_c = true,
            _ => file_path = Some(arg.as_str()),
