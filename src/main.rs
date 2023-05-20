use clap::{Arg, App};
use docx_rs::*;
use serde_json::Value;
use std::io::Read;

const PROHIBITED_WORDS: &[&str] = &["эта", "этот", "эти", "то", "тот", "тех", "такой", "такая", "такие", "такого",
                                    "такому", "таким", "он", "она", "оно", "они", "себя", "сам", "сама", "сами",
                                    "само", "свой", "свои", "своего", "своих", "кто", "что", "какой", "какая", "какие",
                                    "какого", "какому", "каким", "который", "которая", "которые", "которого", 
                                    "которому", "которым", "кем", "чей", "чье", "чья", "чьи", "где", "когда",
                                    "котором", "котором-либо", "котором-нибудь", "каким-либо", "каким-нибудь",
                                    "какой-либо", "какой-нибудь", "\"", " - "];

fn parse_docx(file_name: &str) -> anyhow::Result<()> {
    let data: Value = serde_json::from_str(&read_docx(&read_to_vec(file_name)?)?.json())?;
    if let Some(children) = data["document"]["children"].as_array() {
        children.iter().for_each(read_children);
    }
    Ok(())
}

fn read_children(node: &Value) {
    if let Some(children) = node["data"]["children"].as_array() {
        children.iter().for_each(|child| {
            if child["type"] != "text" {
                read_children(child);
            } else {
                let text = child["data"]["text"].as_str().unwrap_or("");
                let words: Vec<&str> = text.split_whitespace().collect();
                for word in words {
                    if PROHIBITED_WORDS.contains(&word) {
                        println!("{}", word);
                    }
                }
            }
        });
    }
}

fn read_to_vec(file_name: &str) -> anyhow::Result<Vec<u8>> {
    let mut buf = Vec::new();
    std::fs::File::open(file_name)?.read_to_end(&mut buf)?;
    Ok(buf)
}

fn main() -> anyhow::Result<()> {
    let matches = App::new("My Super Program ")
                          .version("1.0")
                          .author("Me <me@example.com>")
                          .about("Does awesome things ")
                          .arg(Arg::with_name("name")
                               .short("n")
                               .long("name")
                               .takes_value(true)
                               .required(true)
                               .help("Sets an input file name"))
                          .get_matches();

    let file_name = matches.value_of("name").unwrap();
    parse_docx(file_name)?;
    Ok(())
}

