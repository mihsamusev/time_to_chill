use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub enum WatchStatus {
    NotStarted,
    Finished,
    Unfinished{comment: String}
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct Movie {
    pub name: String,
    pub status: WatchStatus,
}

pub trait MovieRepository {
    fn get_movies(&self) -> HashMap<String, WatchStatus>;
}

pub struct JsonMovieRepository {
    movies: HashMap<String, WatchStatus>, // Vec<Movie, how to make sure its unique?
}

impl MovieRepository for JsonMovieRepository {
    fn get_movies(&self) -> HashMap<String, WatchStatus> {
        self.movies.clone()
    }
}

impl JsonMovieRepository {
    pub fn new(filename: &str) -> Self {
        let file = File::open(filename).expect("Could not read");
        let json: Vec<Movie> = serde_json::from_reader(file).unwrap();
        let movies: HashMap<String, WatchStatus>= json.into_iter().map(|m| (m.name, m.status)).collect();
        Self {movies}
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
