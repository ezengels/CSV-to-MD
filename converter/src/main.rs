use std::{error::Error, fs::File, fmt, io, process,};
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
        impl fmt::Display for Record {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
               write!(f, "({}, {})", self.layout, self.format)
            }
        }
        let origin = Record { layout: "".to_owned(), format: "".to_owned()};
        println!("{}", origin)
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1)
    }
}