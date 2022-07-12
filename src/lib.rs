use std::{collections::HashMap, io::Error};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum WatchStatus {
    NotStarted,
    Finished,
    Interrupted(String)
}

pub struct Movie {
    pub name: String,
    pub status: WatchStatus
}

pub trait MovieRepository {
    fn get_movies(&self) -> HashMap<String, WatchStatus>;
}

pub struct JsonMovieRepository {
    movies: HashMap<String, WatchStatus>
}

impl JsonMovieRepository {
    pub fn new(filename: &str) -> Self {
        println!("GONNAREAD: {}", filename);
        Self{movies: HashMap::new()}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}