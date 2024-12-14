pub fn test_2() {
    // this tests varifies that a more qaulified person shuld earn more than one taht is not qualified for this I just randomely inputed the values being higher
    let coefficients = vec![
        386.3441, -8362.3122, 294.9915, 11356.3256, 5502.7098,
        7584.1234, 2022.7029, 11879.8013, 11775.4350, -81.8332,
    ];
    let intercept = -3609.6646;

    // Person 1 (less qualified): Lower feature values
    let person1_features = vec![25.0, 0.0, 3.0, 0.0, 1.0, 2.0, 50.0, 0.0, 0.0, 1.0];

    // Person 2 (more qualified): Higher feature values
    let person2_features = vec![35.0, 0.0, 10.0, 1.0, 5.0, 4.0, 80.0, 1.0, 1.0, 0.0];

    // Act
    let person1_salary: f64 = person1_features
        .iter()
        .zip(coefficients.iter())
        .map(|(f, c)| f * c)
        .sum::<f64>() + intercept;

    let person2_salary: f64 = person2_features
        .iter()
        .zip(coefficients.iter())
        .map(|(f, c)| f * c)
        .sum::<f64>() + intercept;

    // Assert
    println!("Person 1 Predicted Salary: {:.6}", person1_salary);
    println!("Person 2 Predicted Salary: {:.6}", person2_salary);
    assert!(
        person2_salary > person1_salary,
        "Expected Person 2's salary ({:.6}) to be greater than Person 1's salary ({:.6})",
        person2_salary,
        person1_salary
    );
}

