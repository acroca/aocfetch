use clap::Parser;
use std::{fs, path::PathBuf, process::exit};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(long)]
  year: u16,

  #[arg(long)]
  day: u8,

  cmd: Vec<String>,
}

fn main() {
  let args = Args::parse();

  match get_input(&args) {
    Ok(contents) => print!("{}", contents),
    Err(e) => {
      eprintln!("{}", e);
      exit(1);
    }
  };
}

fn run(args: &Args) -> Result<(), String> {
  let input = get_input(args)?;

  Ok(())
}

fn get_input(args: &Args) -> Result<String, String> {
  let target_file = input_file(&args)?;
  match fs::read_to_string(&target_file) {
    Ok(c) => Ok(c),
    Err(_) => {
      let downloaded = download(&args)?;
      match std::fs::write(&target_file, &downloaded) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string()),
      };
      Ok(downloaded)
    }
  }
}

fn input_file(args: &Args) -> Result<PathBuf, String> {
  let cache_dir = match dirs::cache_dir() {
    Some(v) => v,
    None => return Err("Impossible to get your cache directory!".into()),
  };

  let cache_dir = cache_dir.join("aoc").join(args.year.to_string());

  match fs::create_dir_all(&cache_dir) {
    Ok(_) => {}
    Err(err) => return Err(err.to_string()),
  };
  Ok(cache_dir.join(args.day.to_string()))
}

fn download(args: &Args) -> Result<String, String> {
  let input_url = format!(
    "https://adventofcode.com/{}/day/{}/input",
    args.year, args.day
  );
  let session = match dotenv::var("AOC_SESSION") {
    Ok(s) => s,
    Err(_) => return Err("`AOC_SESSION` environment variable is not defined".into()),
  };

  let client = reqwest::blocking::Client::new();
  let res = client
    .get(input_url)
    .header("Cookie", format!("session={}", session))
    .send()
    .unwrap();

  match res.text() {
    Ok(t) => Ok(t),
    Err(e) => Err(format!("Error getting input text: {}", e.to_string())),
  }
}
