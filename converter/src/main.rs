use std::{error::Error, fs::File, fmt, io, io::Write, process,};
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
        let content = "{}".to_string("{}", record.layout + "\n" + &record.format);
        let file = File::create("output/".to_owned() + &itemid + ".md")?;
        file.write(content);

       // impl fmt::Display for Record {
       //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       //        write!(f, "({}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {})", 
        //       self.id, self.layout, self.format, self.title, self.group, self.creator, self.creationdate, self.shortdesc, self.copyright, self.teammember, self.contributor)
          //  }
       // }
        //let origin = Record { id:"".to_owned(), layout:"".to_owned(), format:"".to_owned(), title:"".to_owned(), 
        //group:"".to_owned(), creator:"".to_owned(), creationdate:"".to_owned(), shortdesc:"".to_owned(), copyright:"".to_owned(), teammember:"".to_owned(), contributor:"".to_owned(),};
        //println!("{:?}", origin)
    }
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1)
    }
}