use linfa::dataset::DatasetBase;
use linfa::traits::{Fit, Predict};
use linfa_linear::LinearRegression;
use ndarray::{Array2, Array1};
use csv::ReaderBuilder;
use std::error::Error;

pub fn Lr() -> Result<(), Box<dyn Error>> {
    // feature names 
    let feature_names = vec![
        "Age", "Female", "Years", "College", "Certificates",
        "Feedback", "NPS", "Explorer", "Diplomat", "Analyst",
    ];

    // dataset
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("modified_data2.csv")?;
    
    let mut features: Vec<Vec<f64>> = Vec::new();
    let mut targets: Vec<f64> = Vec::new();

    // convert features and targets
    for result in reader.records() {
        let record = result?;
        let feature_row: Vec<f64> = record
            .iter()
            .take(record.len() - 1) // All columns except the last
            .map(|x| x.parse().unwrap())
            .collect();
        let target: f64 = record[record.len() - 1].parse().unwrap(); // Last column is the target
        features.push(feature_row);
        targets.push(target);
    }

    // ndarray (Lecture notes)
    let x: Array2<f64> = Array2::from_shape_vec((features.len(), features[0].len()), features.concat())?;
    let y: Array1<f64> = Array1::from_vec(targets);

    // I have to wrap
    let dataset = DatasetBase::new(x, y);

    // Fit the linear regression model
    let model = LinearRegression::new().fit(&dataset)?;

    // Get the intercept and coefficients
    let intercept = model.intercept();
    let coefficients = model.params();

    // error check just in case
    if coefficients.len() != feature_names.len() {
        panic!(
            "Number of feature names ({}) does not match the number of coefficients ({}).",
            feature_names.len(),
            coefficients.len()
        );
    }

    // Output the intercept and coefficients which will be used for the linear equation
    println!("Intercept: {:.4}", intercept);
    println!("Coefficients:");
    for (i, coeff) in coefficients.iter().enumerate() {
        println!("{}: {:.4}", feature_names[i], coeff);
    }

    Ok(())
}