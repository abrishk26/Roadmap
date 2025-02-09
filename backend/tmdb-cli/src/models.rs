#[derive(serde::Deserialize, std::fmt::Debug)]
pub struct Movie {
    pub title: String,
    pub overview: String,
    pub release_date: String,
    pub vote_average: f64,
}

#[derive(serde::Deserialize, std::fmt::Debug)]
pub struct Res {
    pub page: i64,
    pub results: Vec<Movie>
}