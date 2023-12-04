use minreq;
use std::io;
use std::env;
use std::process::ExitCode;

fn main() -> Result<(), String> {
    let mut cookie = String::new();
    let mut day = "1";
    let mut year = "2023";

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-cookie" | "--cookie" | "-c" => {
                if let Some(new_cookie) = args.next() {
                    cookie = new_cookie;
                } else {
                    return Err(format!("expected string after '{arg}'"));
                }
            },
            _ => return Err(format!("unknown command-line argument '{arg}'")),
        }
    }

    let url = format!("https://adventofcode.com/{year}/day/{day}/input");

    let response = minreq::get(url)
        .send()
        .map_err(|err| format!("could not send request: {err}"))?;

    let body = response
        .as_str()
        .map_err(|err| format!("could not read response: {err}"))?;

    // assert!(body.contains("</html>"));
    assert_eq!(200, response.status_code);
    assert_eq!("OK", response.reason_phrase);

    println!("{}", body);
    Ok(())
}

