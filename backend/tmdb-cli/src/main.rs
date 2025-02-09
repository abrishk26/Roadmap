mod cli;
mod models;
use clap::Parser;
use cli::Mode;
use comfy_table::Table;
use dotenvy::dotenv;
use reqwest::blocking::Client;
use std::env;

fn main() {
    // Load variables
    match dotenv() {
        Ok(_) => (),
        Err(err) => {
            eprintln!("failed to load variables {err}");
            return;
        }
    }

    // Set up the argument parser
    let app = cli::Cli::parse();

    // Retrieve the api-key from the environment
    let api_key: String;
    match env::var("API_KEY") {
        Ok(key) => api_key = key,
        Err(err) => {
            eprintln!("env variable API_KEY must be set: {err}");
            return;
        }
    }
    
    let mut table = Table::new();
    table.set_header(vec!["Title", "Overview", "Rating", "Release Date"]);
    
    match app.r#type {
        Mode::PLAYING => {
            let response = make_request("https://api.themoviedb.org/3/movie/now_playing", &api_key);
            match response {
                Ok(res) => {
                    for m in res.results {
                        table.add_row(vec![m.title, m.overview, m.vote_average.to_string(), m.release_date]);
                    }
                    println!("{table}");
                },
                Err(err) => println!("Error: {err}"),
            }
        }
        Mode::POPULAR => {
            let response = make_request("https://api.themoviedb.org/3/movie/popular", &api_key);
            match response {
                Ok(res) => {
                    for m in res.results {
                        table.add_row(vec![m.title, m.overview, m.vote_average.to_string(), m.release_date]);
                    }
                    println!("{table}");
                },
                Err(err) => println!("Error: {err}"),
            }
        }
        Mode::TOP => {
            let response = make_request("https://api.themoviedb.org/3/movie/top_rated", &api_key);
            match response {
                Ok(res) => {
                    for m in res.results {
                        table.add_row(vec![m.title, m.overview, m.vote_average.to_string(), m.release_date]);
                    }
                    println!("{table}");
                },
                Err(err) => println!("Error: {err}"),
            }
        }
        Mode::UPCOMING => {
            let response = make_request("https://api.themoviedb.org/3/movie/upcoming", &api_key);
            match response {
                Ok(res) => {
                    for m in res.results {
                        table.add_row(vec![m.title, m.overview, m.vote_average.to_string(), m.release_date]);
                    }
                    println!("{table}");
                },
                Err(err) => println!("Error: {err}"),
            }
        }
    }
}

fn make_request(url: &str, api_key: &str) -> Result<models::Res, Box<dyn std::error::Error>> {
    let client = Client::new()
        .get(url)
        .header("Authorization", "Bearer ".to_owned() + api_key);
    let response: models::Res = client.send()?.json()?;
    Ok(response)
}
