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
