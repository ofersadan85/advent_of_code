use anyhow::{Context, Ok, Result};
use chrono::{prelude::*, DateTime, Utc};
use clap::Parser;
use reqwest::Client;
use std::path::PathBuf;
use tracing::{info, instrument};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[arg(short, long)]
    pub year: Option<u16>,
    #[arg(long)]
    pub all_years: bool,
    #[arg(short, long)]
    pub day: Option<u8>,
    #[arg(long)]
    pub all_days: bool,
    #[arg(short, long)]
    pub session: String,
    #[arg(short, long)]
    pub output: PathBuf,
}

#[instrument]
async fn get_input(client: Client, year: u16, day: u8) -> Result<()> {
    info!("Getting input for year {} day {}", year, day);
    let args = Cli::parse();
    let year_str = year.to_string();
    let year_dir = args.output.join(&year_str);
    let filename = format!("day{day:02}.txt");
    let input_path = year_dir.join(&filename);
    if input_path.is_file() {
        info!("Skipping {year}/{filename} - Already exists");
        return Ok(());
    }
    if !year_dir.is_dir() {
        tokio::fs::create_dir_all(&year_dir)
            .await
            .with_context(|| format!("Failed to create directory {}", year_dir.display()))?;
    }

    let url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let response = client
        .get(&url)
        .header("Cookie", format!("session={}", args.session))
        .send()
        .await
        .with_context(|| format!("Failed to get input for day {day}"))?;
    let input = response
        .text()
        .await
        .with_context(|| format!("Failed to get input for day {day}"))?;
    tokio::fs::write(&input_path, input)
        .await
        .with_context(|| format!("Failed to write input to {}", input_path.display()))?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Cli::parse();
    let mut max_date: DateTime<Utc> = Utc::now();
    if max_date.month() < 12 {
        max_date = max_date
            .with_year(max_date.year() - 1)
            .expect("Year is invalid")
            .with_month(12)
            .expect("Month is invalid")
            .with_day(26)
            .expect("Day is invalid");
    }

    let max_year = u16::try_from(max_date.year())?;
    let years = args.year.map_or(2015..=max_year, |y| y..=y);
    let days = args.day.map_or(1..=25, |d| d..=d);
    if years.start() < &2015 || *years.end() > (max_year) {
        eprintln!(
            "Year must be between 2015 and {}, or specify --all_years",
            max_date.year()
        );
        std::process::exit(1);
    }
    if days.start() < &1 || days.end() > &25 {
        eprintln!("Day must be between 1 and 25, or specify --all_days");
        std::process::exit(1);
    }
    if !args.output.is_dir() {
        tokio::fs::create_dir(&args.output)
            .await
            .with_context(|| format!("Failed to create directory {}", args.output.display()))?;
        std::process::exit(1);
    }
    let client = Client::new();
    let mut joinset = tokio::task::JoinSet::new();
    for year in years {
        for day in days.clone() {
            let task = get_input(client.clone(), year, day);
            joinset.spawn(task);
        }
    }
    joinset.join_all().await;
    Ok(())
}
