use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn part_one<R: BufRead>(reader: R) -> Result<(), String> {
    let mut total:i128 = 0;
    for line in reader.lines() {
        let line = line.map_err(|err| format!("Failed reading line: {err}"))?;
        if line.trim().is_empty() {
            continue;
        }
        let ranges = line.split(',');
        for range in ranges {
            match range.split_once('-') {
                Some((a, b)) => {
                    let min = a.parse::<i128>().unwrap();
                    let max = b.parse::<i128>().unwrap();

                    let split_point = a.len().div_euclid(2);
                    let (left, _right) = a.split_at(split_point);
                    let mut i = if left.trim().is_empty() { 1} else {left.parse::<i128>().unwrap()};

                    let mut candidate = (i.to_string() + &i.to_string()).parse::<i128>().unwrap();
                    while candidate <= max {
                        if candidate >= min {
                            total += candidate;
                        }
                        i += 1;
                        candidate = (i.to_string() + &i.to_string()).parse::<i128>().unwrap();
                    }
                },
                None => return Err("expected exactly one space".into()),
            }
        }
    }
    println!("total is {total}");
    Ok(())
}

fn part_two<R: BufRead>(_reader: R) -> Result<(), String> {

    Ok(())
}

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
