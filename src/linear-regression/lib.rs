/*
<----------------------------------
Author: Ilia Sichinava 
Date: December 3rd, 2022
Title: Linear Regression
---------------------------------->
*/
use core::panic;

// Define struct for unknown k and b values
pub struct Equation {
    k: f32,
    b: f32
}


// Returns k and b, prints equation
pub fn linear_regression(data: &Vec<(f32, f32)>) -> Equation {
    // Check for empty data
    if data.len() == 0 {
        panic!("Enter non-zero data")
    }

    // Find everything for our k and b
    let x_mean: f32 = data.iter().map(|point| point.1).sum::<f32>() / data.len() as f32;
    let y_mean: f32 = data.iter().map(|point| point.0).sum::<f32>() / data.len() as f32;
    let xy_mean: f32 = data.iter().map(|point| point.1 * point.0).sum::<f32>() / data.len() as f32;
    let x_square_mean: f32 = data.iter().map(|point| point.1 * point.1).sum::<f32>() / data.len() as f32;

    // ------------------------->
    if x_square_mean - (x_mean * x_mean) == 0.0 {
        panic!("Can't divide by zero")
    }

    // Now let's calculate them
    let k: f32 = (xy_mean - (x_mean * y_mean)) / (x_square_mean - (x_mean * x_mean));
    let b: f32 = y_mean - (k * x_mean);
    
    print!("Regression line: ");
    #[allow(illegal_floating_point_literal_pattern)]
    // Check for k and b to display data correctly. 
    // Print y = x + 3 instead of y = 1x + 3
    match (k, b) {
        // Cases where k = 1
        (1.0, 0.0) => println!("y = x"),
        (1.0, _) => println!("y = x + {}", b),        
        (0.0, _) => println!("y = {}", b),

        // Case where b = 0
        (_, 0.0) => println!("y = {}x", k),
        
        // k != 1 && b != 0
        _ => println!("y = {}x + {}", k, b)
    }
    println!("-----------------------------------------");
    
    // Return found equation members and use them for predictions afterwards
    Equation { k, b }
}

pub fn predict(e: &Equation, x: f32) -> () {
    // Simply count y from y = kx + b equation
    println!("In {} Georgian GDP would be: ${:.3}B\nBut now it is $18.7B", x, e.k * x + e.b);
}