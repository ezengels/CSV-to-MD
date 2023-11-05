use std::{error::Error, fs::File, io, io::prelude::*, process,};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde()]
struct Record {
    id: String,
    layout: String,
    format: String,
    title: String,
    group: String,
    creator: String,
    creationdate: String,
    shortdesc: String,
    copyright: String,
    teammember: String,
    contributor: String,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Record = result?;
        let itemid = record.id;
        let mut file = File::create("output/".to_owned() + &itemid + ".md")?;
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1)
    }
}