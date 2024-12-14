use csv::{ReaderBuilder, WriterBuilder, Trim};
use std::error::Error;
use std::fs::{self, File};
use std::path::Path;

pub fn clean2() -> Result<(), Box<dyn Error>> {
    let input_path = "modified_data.csv";
    let output_path = "modified_data2.csv";

    // Ensure the directory exists for the output file
    if let Some(parent) = Path::new(output_path).parent() {
        fs::create_dir_all(parent)?;
    }

    // Open the input CSV file.
    let file = File::open(input_path)?;
    let mut rdr = ReaderBuilder::new()
        .trim(Trim::All)
        .from_reader(file);

    // Create the output CSV file.
    let output_file = File::create(output_path)?;
    let mut wtr = WriterBuilder::new().from_writer(output_file);

    // Read headers and re-order them so "Salary" is the last column
    let headers = rdr.headers()?.clone();
    let columns_to_drop = vec!["Sales_Rep", "Business", "Personality"];
    let mut indexes_to_keep: Vec<usize> = Vec::new();
    let mut headers_to_write: Vec<&str> = Vec::new();
    let mut salary_index: Option<usize> = None;

    for (index, header) in headers.iter().enumerate() {
        if header == "Salary" {
            salary_index = Some(index); // Store the index of the Salary column
        } else if !columns_to_drop.contains(&header) {
            indexes_to_keep.push(index);
            headers_to_write.push(header);
        }
    }

    // Add "Salary" header at the end if it exists
    if let Some(index) = salary_index {
        indexes_to_keep.push(index); // Ensure Salary index is the last in the list
        headers_to_write.push("Salary");
    }

    // Write reordered headers to the output file.
    wtr.write_record(&headers_to_write)?;

    // Iterate over each record, reordering so "Salary" is the last column.
    for result in rdr.records() {
        let record = result?;
        let mut record_to_write: Vec<&str> = Vec::new();

        // Collect data in the new order, except Salary
        for &index in &indexes_to_keep {
            record_to_write.push(record.get(index).unwrap_or(""));
        }

        // Write the reordered record to the output file
        wtr.write_record(&record_to_write)?;
    }

    wtr.flush()?;
    Ok(())
}
