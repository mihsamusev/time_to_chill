use std::{collections::HashMap, env};
use time_to_chill::{JsonMovieRepository, WatchStatus, pick_unwatched, MovieRepository};

fn parse_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args
}

fn main() {
    let args = parse_args();
    let filename = &args[1];
    let repo = JsonMovieRepository::new(filename);

    // enter UI loop
    // greetings and navigation

    // modes (all, unwatched only, unwatched or unfinished)
    // include unfinished?

    // rewatch?

    // You have following unfinished movies
    // Movie / reason, would you like to randomly choose among then?

    // Youre chilln to:
    let movie = pick_unwatched(&repo.get_movies());
    match movie {
        None => println!("We're watching nothing bae"),
        Some(m) => println!("we're watchng {} bae", m)
    };
}
