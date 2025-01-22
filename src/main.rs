use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;

use clap::Parser;
use encoding_rs::Encoding;

mod argument;


fn bin_parser(mut file: File, decoder: &'static Encoding, min_length: usize) -> Vec<String> {
    let mut res = Vec::new();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();
    let (string, _, _) = decoder.decode(&data);
    for part in string.split("�") {
        if part.len() < min_length {
            continue;
        }
        let s = part.chars().filter(|x| !x.is_control()).collect::<String>();
        if s.len() > min_length {
            res.push(s);
        }
    }
    res
}

fn regex_filter(data: Vec<String>, re: regex::Regex) -> Vec<String> {
    data.into_iter().filter(|x| re.is_match(x)).collect()
}


fn run() -> Result<(), Box<dyn Error>> {
    let args: argument::CliArguments = argument::CliArguments::parse();
    let file = File::open(args.file)?;
    let data = bin_parser(file, args.decoder, args.min_length);
    let data = if let Some(re) = args.re { regex_filter(data, re) } else { data };
    for line in data.iter() {
        println!("{}", line);
    }
    if let Some(path) = args.output {
        let mut file = File::create(path)?;
        for line in data {
            writeln!(file, "{}", line)?;
        }
    }
    Ok(())
}

fn main() {
    let r = run();
    if let Err(e) = r {
        eprintln!("{}", e);
        exit(1)
    }
}