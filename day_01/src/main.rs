use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

struct CliArgs {
    input: String,
    part: String,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = parse_args(env::args().skip(1))?;
    let file =
        File::open(&args.input).map_err(|err| format!("Failed to open {}: {err}", args.input))?;
    let reader = BufReader::new(file);

    println!("Running part {}", args.part);
    match args.part.as_str() {
        "one" => {
            part_one(reader)?;
            Ok(())
        }
        "two" => {
            part_two(reader)?;
            Ok(())
        }
        other => Err(format!("Unknown part '{other}', expected 1 or 2")),
    }
}

fn part_one<R: BufRead>(reader: R) -> Result<(), String> {
    let mut dial: i128 = 50;
    let mut count_zero = 0;

    for line in reader.lines() {
        let line = line.map_err(|err| format!("Failed reading line: {err}"))?;
        let value = parse_signed_value(&line)?;

        // we treat dial like an unsigned integer that goes from 0 to 99 by using rem_euclid on
        // dial + value. [Calculates the least nonnegative remainder of self (mod v).]
        dial = (dial + value).rem_euclid(100);
        println!("{value} -> {dial}");
        if dial == 0 {
            count_zero += 1;
        }
    }
    println!("Zero count: {count_zero}");
    Ok(())
}

fn part_two<R: BufRead>(reader: R) -> Result<(), String> {
    let mut dial: i128 = 50;
    let mut count_zero = 0;

    for line in reader.lines() {
        let line = line.map_err(|err| format!("Failed reading line: {err}"))?;
        let value = parse_signed_value(&line)?;
        println!("Dial is at {dial}");
        println!("The dial is rotated {line} to point at {point}", line = line, point = (dial + value).rem_euclid(100));

        if dial == 0 {
            // we start at zero, so just count total rotations
            count_zero += (dial + value).abs() / 100;
        } else if dial + value <= 0 {
            // +1 when we pass zero for the first time
            count_zero += 1 + (dial + value).abs() / 100;
        } else if dial + value >= 100 {
            count_zero += (dial + value) / 100;
        }
        dial = (dial + value).rem_euclid(100);
    }

    println!("Zero count: {count_zero}");
    Ok(())
}

fn parse_args<I>(mut args: I) -> Result<CliArgs, String>
where
    I: Iterator<Item = String>,
{
    let mut input = None;
    let mut part = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" | "-i" => {
                let Some(path) = args.next() else {
                    return Err("Missing value for --input".to_string());
                };

                input = Some(path);
            }
            "--part" | "-p" => {
                let Some(value) = args.next() else {
                    return Err("Missing value for --part".to_string());
                };

                part = Some(value);
            }
            _ => {
                if let Some(path) = arg.strip_prefix("--input=") {
                    input = Some(path.to_string());
                    continue;
                }

                if let Some(value) = arg.strip_prefix("--part=") {
                    part = Some(value.to_string());
                    continue;
                }

                return Err(format!("Unknown argument '{arg}'"));
            }
        }
    }

    let input = input.ok_or_else(|| "Missing required --input <path>".to_string())?;
    let part = part.ok_or_else(|| "Missing required --part <value>".to_string())?;

    Ok(CliArgs { input, part })
}

fn parse_signed_value(raw: &str) -> Result<i128, String> {
    let trimmed = raw.trim();
    let mut chars = trimmed.chars();

    // prefix stores our L or R
    let Some(prefix) = chars.next() else {
        return Err("Empty line in input".to_string());
    };

    // rest of string is our nubmer
    let number_str: String = chars.collect();
    let magnitude: i128 = number_str
        .parse()
        .map_err(|err| format!("Invalid number in input '{trimmed}': {err}"))?;

    // we return the number as negative for L and positive for R
    match prefix {
        'L' => Ok(-magnitude),
        'R' => Ok(magnitude),
        _ => Err(format!(
            "Invalid direction prefix in input '{trimmed}': expected L or R"
        )),
    }
}
