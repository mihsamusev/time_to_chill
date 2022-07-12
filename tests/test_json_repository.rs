use serde::ser::{Serialize, SerializeMap, Serializer};
use serde_json;
use std::collections::HashMap;
use time_to_chill::{self, MovieRepository, WatchStatus};

struct TestMovieRepository {}

impl MovieRepository for TestMovieRepository {
    fn get_movies(&self) -> HashMap<String, WatchStatus> {
        HashMap::from([
            ("Cowboy Bebop".into(), WatchStatus::NotStarted),
            ("Snatch".into(), WatchStatus::NotStarted),
            ("Inglorious Bastards".into(), WatchStatus::Finished),
        ])
    }
}

impl Serialize for TestMovieRepository {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut movies: Vec<(String, WatchStatus)> = self.get_movies().into_iter().collect();
        movies.sort_by(|x, y| x.0.cmp(&y.0));
        let mut seq = serializer.serialize_map(Some(movies.len()))?;
        for (k, v) in &movies {
            seq.serialize_entry(k, v)?;
        }
        seq.end()
    }
}

#[test]
fn test_serialize_repo() {
    let original = "\
    {\
        \"Cowboy Bebop\":\"NotStarted\",\
        \"Inglorious Bastards\":\"Finished\",\
        \"Snatch\":\"NotStarted\"\
    }";
    let repo = TestMovieRepository {};
    let serialized = serde_json::to_string(&repo).unwrap();
    assert_eq!(original, serialized)
}

#[test]
fn test_pick_not_started_movie() {
    let repo = TestMovieRepository {};
    let movies = repo.get_movies();
    let movie = time_to_chill::pick_unwatched(&movies);
    assert!(movie == Some("Cowboy Bebop".into()) || movie == Some("Snatch".into()))
}
