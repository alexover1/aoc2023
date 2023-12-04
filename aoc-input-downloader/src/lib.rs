use std::io::Write;
use std::fs::{self, File};
use minreq;
use anyhow::{anyhow, Result, Context};

pub const DEFAULT_INPUT_PATH: &str = "input";

pub fn get_input(day: usize, year: &String, cookie: &String) -> Result<String> {
    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    let response = minreq::get(&url)
        .with_header("Cookie", format!("session={cookie}"))
        .send()
        .with_context(|| format!("Could not send request: {}", url))?;

    let body = response
        .as_str()
        .context(format!("Could not read response"))?;

    if response.status_code != 200 {
        return Err(anyhow!(format!("Response status code {}", response.status_code)));
    }

    assert_eq!("OK", response.reason_phrase);

    Ok(body.to_string())
}

pub fn get_all_inputs(days: Vec<usize>, year: &String, cookie: &String, output_dir: &String) -> Result<()> {
    let mut all_ok = true;

    fs::create_dir_all(&output_dir)
        .with_context(|| format!("Could not create output directory: {}", output_dir))?;

    println!("Downloading inputs:");

    'next_day: for day in days {
        let input = match get_input(day, &year, &cookie) {
            Ok(input) => input,
            Err(e) => {
                eprintln!("    Day {:2}: {}", day, e);
                all_ok = false;
                continue 'next_day;
            },
        };

        let file_path = format!("{output_dir}/{day}.txt");
        let mut file = File::create(&file_path)
            .with_context(|| format!("Could not create file: {}", file_path))?;

        file.write_all(input.as_bytes()).context("Could not write file")?;

        println!("    Day {:2}: Created file {}", day, file_path);
    }

    if all_ok {
        println!("--- All OK ---");
    } else {
        println!("--- Not all inputs were downloaded successfully, but that may be expected if they have not been released yet. ---");
    }

    Ok(())
}
