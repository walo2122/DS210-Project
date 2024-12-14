pub fn test_1() {
    // Coefficients and intercept
    let coefficients = vec![
        386.3441, -8362.3122, 294.9915, 11356.3256, 5502.7098,
        7584.1234, 2022.7029, 11879.8013, 11775.4350, -81.8332,
    ];
    let intercept = -3609.6646;

    // Feature values (all zeros)
    let feature_values = vec![0.0; coefficients.len()]; // All feature values set to 0.0 so that the output should be the intercept

    // Calculate predicted salary
    let predicted_salary: f64 = feature_values
        .iter()
        .zip(coefficients.iter())
        .map(|(f, c)| f * c)
        .sum::<f64>() + intercept;

    // Output the result
    println!("Predicted Salary if equation works should be -3609.664600: {:.6}", predicted_salary);
}


    