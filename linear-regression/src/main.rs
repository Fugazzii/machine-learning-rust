use linear_regression::{linear_regression, predict};

fn main() {
    // Test data

    let data: Vec<(f32, f32)> = vec![
        (17.9, 2013.0),
        (17.63, 2014.0),
        (14.95, 2015.0),
        (15.14, 2016.0),
        (16.24, 2017.0),
        (17.6, 2018.0),
        (17.47, 2019.0),
        (15.84, 2020.0),
        (18.7, 2021.0)
    ];


    for price in &data {
        println!("Year: {}, GDP = ${:.3}B", price.1, price.0);
    }

    // Linear regression prints the equation and returns k and b
    let eq = linear_regression(&data);

    // Test cases for different x values
    predict(&eq, 2022.0);
}