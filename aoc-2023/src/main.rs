use std::io;
use std::mem;
use std::env;
use std::path::Path;
use aoc_input_downloader::{get_all_inputs, DEFAULT_INPUT_PATH};
use anyhow::{anyhow, Result, Context};

use day_01;
use day_02;
use day_03;
use day_04;

fn main() -> Result<()> {
    let mut input_dir: Option<String> = None;
    let mut cookie = String::new();

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-cookie" | "--cookie" | "-c" => {
                cookie = args.next().context(format!("Expected string after argument: {}", arg))?;
            },
            "--input_dir" | "-input_dir" | "--dir" | "-dir" => {
                let new = args.next().context(format!("Expected string after argument: {}", arg))?;
                
                if let Some(old) = mem::replace(&mut input_dir, Some(new.clone())) {
                    return Err(anyhow!(format!("Duplicate input directory flag: {} and {}", new, old)));
                }
                
            },
            _ => return Err(anyhow!(format!("Invalid command-line argument: {}", arg))),
        }
    }

    let input_dir = input_dir.unwrap_or(DEFAULT_INPUT_PATH.to_string());

    if !Path::new(&input_dir).exists() {
        if cookie.is_empty() {
            println!("Please enter session cookie:");
            io::stdin().read_line(&mut cookie).context("Could not read input")?;
            cookie = cookie.trim().to_string();
        }

        let year = "2023".to_string();
        get_all_inputs((1..=4).collect(), &year, &cookie, &input_dir)?;
    }

    day_01::run_all();
    day_02::run_all();
    day_03::run_all();
    day_04::run_all();

    Ok(())
}
