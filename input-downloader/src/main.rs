use std::io;
use std::env;
use std::mem;
use input_downloader::{get_all_inputs, DEFAULT_INPUT_PATH};
use anyhow::{anyhow, Result, Context};

fn main() -> Result<()> {
    let mut cookie = String::new();
    let mut days: Vec<usize> = Vec::new();
    let mut year = "2023".to_string();

    let mut output_dir: Option<String> = None;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-cookie" | "--cookie" | "-c" => {
                cookie = args.next().context(format!("Expected string after argument: {}", arg))?;
            },
            "--year" | "-year" | "-y" => {
                year = args.next().context(format!("Expected string after argument: {}", arg))?;
            },
            "--day" | "-day" | "-d" => {
                let arg = args
                    .next()
                    .context(format!("Expected string after argument: {}", arg))?;

                let range = parse_range(&arg).with_context(|| format!("Failed to parse range: {}", arg))?;
                for num in range {
                    days.push(num);
                }
            },
            "--output_dir" | "-output_dir" | "--dir" | "-dir" => {
                let new = args.next().context(format!("Expected string after argument: {}", arg))?;
                
                if let Some(old) = mem::replace(&mut output_dir, Some(new.clone())) {
                    return Err(anyhow!(format!("Duplicate output directory flag: {} and {}", new, old)));
                }
            },
            _ => return Err(anyhow!(format!("Invalid command-line argument: {}", arg))),
        }
    }

    if cookie.is_empty() {
        println!("Please enter session cookie:");
        io::stdin().read_line(&mut cookie).context("Could not read input")?;
        cookie = cookie.trim().to_string();
    }

    let output_dir = output_dir.unwrap_or(DEFAULT_INPUT_PATH.to_string());
    if days.len() == 0 {
        get_all_inputs((1..=25).collect(), &year, &cookie, &output_dir)?;
    } else {
        get_all_inputs(days, &year, &cookie, &output_dir)?;
    }

    Ok(())
}

fn parse_range(input: &str) -> Result<std::ops::RangeInclusive<usize>> {
    let parts: Vec<&str> = input.split("..").collect();

    match parts.len() {
        1 => {
            let single_value = parts[0].parse::<usize>().context("Invalid number")?;
            Ok(single_value..=single_value)
        },
        2 => {
            let start = parts[0].parse::<usize>().context("Invalid start number")?;
            let end = parts[1].parse::<usize>().context("Invalid end number")?;
            Ok(start..=end)
        },
        _ => Err(anyhow!("Invalid range format")),
    }
}
