use plotters::prelude::*;
use csv::Reader;
use std::collections::HashMap;
use std::error::Error;

pub fn graph() -> Result<(), Box<dyn Error>> {
    let file_path = "modified_data2.csv";
    let output_path = "graph.png";

    // Open CSV and read headers
    let mut reader = Reader::from_path(file_path)?;
    let headers = reader.headers()?;
    let nps_index = headers.iter().position(|h| h == "NPS").ok_or("NPS column not found")?;
    let salary_index = headers.iter().position(|h| h == "Salary").ok_or("Salary column not found")?;

    // Aggregate data
    let mut data_map: HashMap<i32, Vec<u32>> = HashMap::new();
    for result in reader.records() {
        let record = result?;
        let nps = record.get(nps_index).ok_or("NPS value missing")?.parse::<i32>()?;
        let salary = record.get(salary_index).ok_or("Salary value missing")?.parse::<u32>()?;
        data_map.entry(nps).or_default().push(salary);
    }

    // Calculate average salary for each unique NPS
    let mut data: Vec<(i32, u32)> = data_map
        .into_iter()
        .map(|(nps, salaries)| {
            let avg_salary = salaries.iter().sum::<u32>() / salaries.len() as u32;
            (nps, avg_salary)
        })
        .collect();

    // Sort data by NPS
    data.sort_by_key(|&(nps, _)| nps);

    // Determine dynamic ranges
    let nps_min = data.iter().map(|&(nps, _)| nps).min().unwrap_or(0);
    let nps_max = data.iter().map(|&(nps, _)| nps).max().unwrap_or(10);
    let salary_min = data.iter().map(|&(_, salary)| salary).min().unwrap_or(0);
    let salary_max = data.iter().map(|&(_, salary)| salary).max().unwrap_or(100000);

    // Plotting
    let root = BitMapBackend::new(output_path, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("NPS vs. Salary (Fixed)", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(nps_min..nps_max, salary_min..salary_max)?;

    chart.configure_mesh().draw()?;

    // Draw line graph
    chart.draw_series(LineSeries::new(
        data.iter().cloned(),
        &BLUE,
    ))?;

    println!("Fixed line graph saved to: {}", output_path);

    Ok(())
}



