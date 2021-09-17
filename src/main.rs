use structopt::StructOpt;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    matched_idx: u8,
    /// The pattern to look for
    matched_key: String,
    /// The pattern to look for
    expore_idx: u8,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
    /// The pattern to look for
    split: Option<String>,
}

fn seek_comp(split: &str, line: &str, matched_idx: usize, matched_key: &String, expore_idx: usize) {
    let split = line.split(split);
    let vec: Vec<&str> = split.collect();

    if let Some(comp) = vec.get(matched_idx) {
        if comp.contains(matched_key) {
            if let Some(result) = vec.get(expore_idx) {
                println!("get expore comp: {}", result);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = Cli::from_args();

    let mut split = "\t";
    if let Some(arg_split) = &args.split {
        split = arg_split;
    }
    
    let f = File::open(&args.path)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        match line {
            Ok(v) => {
                seek_comp(&split, &v, args.matched_idx as usize, &args.matched_key, args.expore_idx as usize);
            },
            Err(e) => { return Err(e) }
        }
    }
    
    Ok(())
}
