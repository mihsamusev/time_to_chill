use std::{collections::HashMap, env};
use time_to_chill::{JsonMovieRepository, WatchStatus};

fn parse_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}

fn main() {
    let args = parse_args();
    let filename = &args[1];
    let repo = JsonMovieRepository::new(filename);
}
