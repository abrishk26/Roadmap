#[derive(serde::Deserialize, std::fmt::Debug)]
pub struct Movie {
    adult: bool,
    id: i64,
    title: String,
    overview: String,
    popularity: f64,
    release_date: String,
    vote_average: f64,
    vote_count: i64
}

#[derive(serde::Deserialize, std::fmt::Debug)]
pub struct Res {
    page: i64,
    results: Vec<Movie>
}