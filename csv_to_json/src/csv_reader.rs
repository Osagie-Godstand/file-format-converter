extern crate csv;
use std::error::Error;
use std::fs::File;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}

pub fn read_csv_file(csv_file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let file = File::open(csv_file_path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(file);

    let mut records = Vec::new();

    for result in rdr.deserialize::<Record>() {
        let record: Record = result?;
        let trimmed_record = Record {
            first_name: record.first_name.trim().to_string(),
            last_name: record.last_name.trim().to_string(),
            address: record.address.trim().to_string(),
            city: record.city.trim().to_string(),
            state: record.state.trim().to_string(),
            zip: record.zip.trim().to_string(),
        };

        records.push(trimmed_record);
    }

    Ok(records)
}

