use std::error::Error;
use std::fs::File;
use std::io::Write;

pub fn write_json_to_file(json: &str, json_file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut output_file = File::create(json_file_path)?;
    output_file.write_all(json.as_bytes())?;

    Ok(())
}

