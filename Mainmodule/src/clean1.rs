use csv::{ReaderBuilder, WriterBuilder};
use std::error::Error;
use std::fs::File;

pub fn clean1() -> Result<(), Box<dyn Error>> {
    // csv file
    let input_file = "TechSales_Rep Hardware Division _PS1.csv";
    let output_file = "modified_data.csv";

    // Open the input CSV file
    let file = File::open(input_file)?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Read the headers and initialize a new header list
    let headers = rdr.headers()?.clone();
    let mut new_headers: Vec<String> = headers.iter().map(|h| h.to_string()).collect();

    // create dummy variables like in exel
    new_headers.push("Explorer".to_string());
    new_headers.push("Diplomat".to_string());
    new_headers.push("Analyst".to_string());

    // add the new headers
    let mut wtr = WriterBuilder::new().from_path(output_file)?;
    wtr.write_record(&new_headers)?;

    // Process each record and modify as needed
    for result in rdr.records() {
        let record = result?;
        let mut new_record: Vec<String> = record.iter().map(|r| r.to_string()).collect();

        // Use maping (lecture notes)
        let personality = new_record[6].to_lowercase(); // Assuming "Personality" is column 6
        new_record.push(if personality == "explorer" { "1" } else { "0" }.to_string());
        new_record.push(if personality == "diplomat" { "1" } else { "0" }.to_string());
        new_record.push(if personality == "analyst" { "1" } else { "0" }.to_string());

        // Convert "College" column (column 5) to binary (1 and 0s will be needed)
        let college_index = 5; //collee index  = 5
        new_record[college_index] = if new_record[college_index] == "Yes" { "1".to_string() } else { "0".to_string() };

        // Write the updated record to the output file
        wtr.write_record(&new_record)?;
    }

    Ok(())
}
