use structopt::StructOpt;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::str;
use serde::{Deserialize, Serialize};
use memmap::MmapOptions;

#[derive(Serialize, Deserialize)]
struct MatchFile {
  name: String,
  comps: Vec<u8>,
  matched_idx: u8,
  begin_idx: Option<u8>,
  end_idx: Option<u8>,
  split: Option<String>
}

#[derive(Serialize, Deserialize)]
struct OriFile {
  comps: Vec<u8>,
  matched_idx: u8,
  begin_idx: Option<u8>,
  end_idx: Option<u8>,
  split: Option<String>
}

#[derive(Serialize, Deserialize)]
struct Conf {
    match_file: MatchFile,
    ori_file: OriFile,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "fast tools")]
enum Command {
    Query {
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
    },
    Replace {
        /// The path to the file to read
        #[structopt(parse(from_os_str))]
        path: std::path::PathBuf,
        /// The path to the file to read
        #[structopt(parse(from_os_str))]
        json_path: Option<std::path::PathBuf>,
    }
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
    let ops = Command::from_args();

    match ops {
        Command::Query { matched_idx, matched_key, expore_idx, path, split } => {
            let mut lsplit = "\t";
            if let Some(arg_split) = &split {
                lsplit = arg_split;
            }
            
            let f = File::open(&path)?;
            let reader = BufReader::new(f);
            for line in reader.lines() {
                match line {
                    Ok(v) => {
                        seek_comp(&lsplit, &v, matched_idx as usize, &matched_key, expore_idx as usize);
                    },
                    Err(e) => { return Err(e) }
                }
            }
        },
        Command::Replace { path, json_path } => {
            let mut jsonf = std::path::PathBuf::from("./conf.json");
            if let Some(f) = json_path {
                jsonf = f;
            }

            let mut f = File::open(jsonf)?;
            let mut reader = BufReader::new(f);            
            let conf: Conf = serde_json::from_reader(reader)?;

            let mut mf_split = "\t".to_owned();
            if let Some(st) = conf.match_file.split {
                mf_split = st;
            };

            f = File::open(conf.match_file.name)?;
            reader = BufReader::new(f);

            let mut matchMap: HashMap<String, Vec<String>> = HashMap::new();
            for line in reader.lines() {
                match line {
                    Ok(vline) => {
                        let split = &vline.split(&mf_split);
                        let vec: Vec<&str> = split.to_owned().collect();
                        
                        let mut comps = vec![];
                        for idx in &conf.match_file.comps {
                            comps.push(vec[*idx as usize].to_owned());
                        }
                        let key: String = vec[conf.match_file.matched_idx as usize].to_owned();
                        matchMap.insert(key, comps);
                    },
                    Err(e) => {
                        return Err(e)
                    }
                }             
            }

            println!("{:?}", matchMap);

            let mut ori_split = "\t".to_owned();
            if let Some(fsplit) = conf.ori_file.split {
                ori_split = fsplit;
            }

            // replaced
            f = File::open(&path)?;
            let mmap = unsafe { MmapOptions::new().map(&f)? };

            let mut outVec = Vec::<u8>::new();
            let mut vVec: Vec<String> = vec![];

            if let Some(spt) = ori_split.bytes().next() {
                for line in mmap.split(|ch| *ch == b'\n') {
                    let mut idx = 0;
                    let mut matched = false;
                    for comps in line.split(|chr| *chr == spt) {
                        let s = match str::from_utf8(comps) {
                            Ok(v) => v,
                            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
                        };
                        
                        if idx == conf.ori_file.matched_idx {
                            let ori_key = s;
                            if let Some(_vVec) = matchMap.get(ori_key) {
                                matched = true;
                                vVec = _vVec.clone();
                            }
                        }

                        if matched == true && conf.match_file.comps.contains(&idx) {
                            println!("result: {}", s);
                            if !vVec.is_empty() {
                                let comp = vVec.remove(0);
                                outVec.extend_from_slice(&comp.as_bytes().to_vec());
                            }
                        } else {
                            outVec.extend_from_slice(comps);
                        }
                        outVec.extend_from_slice(&[spt]);
                        idx += 1;
                    }
                    outVec.extend_from_slice(&[b'\n']);
                }
            }
            
            let out_file_name = format!("{}.tmp", path.into_os_string().into_string().unwrap());
            let mut outFile = File::create(out_file_name)?;
            outFile.write_all(&outVec); 
        },
    }    

    Ok(())
}
