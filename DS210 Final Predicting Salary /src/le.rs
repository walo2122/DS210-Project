use std::error::Error;

pub fn Le() -> Result<(), Box<dyn Error>> {
    // feature names and coefficients 
    let feature_names = vec![
        "Age", "Female", "Years", "College", "Certificates",
        "Feedback", "NPS", "Explorer", "Diplomat", "Analyst",
    ];
    let coefficients = vec![
        386.3441, -8362.3122, 294.9915, 11356.3256, 5502.7098,
        7584.1234, 2022.7029, 11879.8013, 11775.4350, -81.8332,
    ];
    let intercept = -3609.6646;

    // empty vector to store user inputs
    let mut feature_values = Vec::new();

    // inputs for each feature
    println!("Enter values for the following features(Female = 1 or 0 for male , College = 1 or 0 for no college and for personality 1 or 0 for other):");
    for feature_name in &feature_names {
        println!("{}: ", feature_name);
        let value: f64 = text_io::read!(); // Read a floating-point number
        feature_values.push(value);
    }

    // predicted salary equation
    let mut predicted_salary = intercept;
    for (i, &value) in feature_values.iter().enumerate() {
        predicted_salary += coefficients[i] * value;
    }

    // Output of predicted salary
    println!("\nPredicted Salary: {:.2}", predicted_salary);

    Ok(())
}