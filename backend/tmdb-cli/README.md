# TMDB CLI TOOL
This project is a command-line utility written in **_Rust_** that uses **TMDB API** to fetch movie information and display it in the terminal.

## Features
- **Now Playing Movies**
- **Popular Movies**
- **Top Rated Movies**
- **UpComing Movies**

## Prerequisite
- Rust(version 1.83.0 or later)
- TMDB API_KEY 


## Installation
1. Clone this repository:
   ```
   git clone github.com/abrishk26/Roadmap.git
   cd Roadmap/backend/tmdb-cli
   ```
2. Create .env file in the root directory with your API_KEY
   ```
   touch .env
   API_KEY=YOUR_API_KEY
   ```

## Usage
   ```
   # Show help and available commands
     cargo run -- --help
   
   # Show movies that are now playing in cinemas.
     cargo run -- --type playing
     
   # Show movies that are popular now.
     cargo run -- --type popular
     
   # Show movies that are top rated.
     cargo run -- --type top
     
   # Show movies that are incoming to the cinemas.
     cargo run -- --type incoming
   ```

## Extras
This Repo serves as a solution to [Roadmap.sh TMDB CLI TOOL Problem](https://roadmap.sh/projects/tmdb-cli)
