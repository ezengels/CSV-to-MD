use std::{error::Error, fs::File, io, io::Write, process,};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde()]
struct Record {
    id: String,layout: String, format: String, title: String,
    contributor: String, group: String, creator: String, creationdate: String,
    shortdesc: String, copyright: String, categories: String,
    medium: String, demographic: String, time: String,
    tags: String, teammember: String,
}

fn run() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Record = result?;
        let itemid = record.id;
        let mut file = File::create("output/".to_owned() + &itemid + ".md")?;
        write!(file, "---\nlayout:{}\nformat:\"{}\"\ntitle:\"{}\"\ncontributor:\"{}\"\ngroup:{}\ncreator:\"{}\"\ncreationdate:\"{}\"\n shortdesc:\"{}\"\n copyright:\"{}\"\nteammember:{}\ncategories:{}\nmedium:{}\ndemographic:{}\ntime:{}\ntags:{}\nteammember:{}\n---",
        record.layout, record.format, record.title, record.group, record.creator, 
        record.creationdate, record.shortdesc, record.copyright, record.teammember, 
        record.contributor, record.categories, record.medium, record.demographic,
        record.time, record.tags, record.teammember).ok();
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1)
    }
}