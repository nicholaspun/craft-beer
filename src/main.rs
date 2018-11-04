extern crate csv;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::io;

#[derive(Debug, Deserialize)]
struct BeerEntry {
    #[serde(rename = "Date")] date: String,
    #[serde(rename = "Brewery")] brewery: String,
    #[serde(rename = "Name")] name: String,
    #[serde(rename = "Description")] description: String,
    #[serde(rename = "Aroma")] aroma: String,
    #[serde(rename = "Taste")] taste: String,
    #[serde(rename = "Finish")] finish: String,
    #[serde(rename = "Rating")] rating: u8
}

fn main() {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut beer_results = Vec::new();
    for result in rdr.deserialize() {
        let record: BeerEntry = result.expect("Can't parse result");
        beer_results.push(record)
    }
}
