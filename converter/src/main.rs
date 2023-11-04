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
            println!("{:?}", record.id)
       // let itemid = Record::id.to_string()
       // let mut file = File::create(itemid + ".md")?;
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1)
    }
}