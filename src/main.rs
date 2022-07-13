use time_to_chill::{JsonMovieRepository, pick_unwatched, MovieRepository};
use clap::{Parser, ArgEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
enum MovieSource {
    GoogleKeep,
    Json,
}

/// Random movie selector for Netflix & Chill evening
#[derive(Parser)]
#[clap(author, version, about, long_about=None)]
struct Args {
    /// Path to the movie collection '.json' file
    #[clap(long, value_parser, default_value_t = String::from("./movies.json"))]
    movie_file: String,
    
    /// Whether to include unfinished movies into the selection pool
    #[clap(long, action, default_value_t = true)]
    include_unfinished: bool,
    
    /// Where to fetch movie collection from
    #[clap(arg_enum, long, default_value_t = MovieSource::Json)]
    movie_source: MovieSource
}

fn main() {
    let args = Args::parse();

    // match args.movie_source {
    //     MovieSource::GoogleKeep => println!("Gonna try to sync GoogleKeep with local movie file"),
    //     MovieSource::Json => println!("Gonna try from local movie file")
    // }
    let repo = JsonMovieRepository::new(&args.movie_file);

    println!("Let me sellect something for you from the list...");

    let movie = pick_unwatched(&repo.get_movies());
    match movie {
        None => println!("🙄 Im afraid there is nothing to watch bae, 😏 but we could go straight to chilling..."),
        Some(m) => println!("Ooo weee 😏, we're watchng <{}> bae 🤤, so you better get comfortable!", m)
    };
    println!("Have fun!")
}
