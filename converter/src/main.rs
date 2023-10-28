use std::{error::Error, io, process};

fn csvreader() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for result in rdr.records() {
        let record = result?;

        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = csvreader() {
        println!("error running csvreader: {}", err);
        process::exit(1);
    }
}
