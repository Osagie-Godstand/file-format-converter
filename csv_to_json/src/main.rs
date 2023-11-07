mod csv_reader;
mod json_writer;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let csv_file_path = "/Users/dgodstand/Downloads/record.csv";
    let json_file_path = "/Users/dgodstand/Downloads/record.json";

    let records = csv_reader::read_csv_file(csv_file_path)?;
    let json = serde_json::to_string_pretty(&records)?;
    json_writer::write_json_to_file(&json, json_file_path)?;

    println!("CSV to JSON conversion complete.");

    Ok(())
}


