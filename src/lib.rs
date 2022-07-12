use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum WatchStatus {
    NotStarted,
    Finished,
    Interrupted(String),
}

pub struct Movie {
    pub name: String,
    pub status: WatchStatus,
}

pub trait MovieRepository {
    fn get_movies(&self) -> HashMap<String, WatchStatus>;
}

pub struct JsonMovieRepository {
    _movies: HashMap<String, WatchStatus>, // Vec<Movie, how to make sure its unique?
}

impl JsonMovieRepository {
    pub fn new(filename: &str) -> Self {
        println!("GONNAREAD: {}", filename);
        Self {
            _movies: HashMap::new(),
        }
    }
}

pub fn pick_unwatched(movies: &HashMap<String, WatchStatus>) -> Option<String> {
    let unwatched_names: Vec<String> = movies
        .iter()
        .filter(|m| *m.1 != WatchStatus::Finished)
        .map(|x| x.0.to_owned())
        .collect();

    let movie_name = unwatched_names.choose(&mut rand::thread_rng());
    match movie_name {
        None => None,
        Some(name) => Some(String::from(name)),
    }
}
