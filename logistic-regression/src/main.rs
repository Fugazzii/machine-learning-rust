use logistic_regression::LogisticRegression;

fn main() {
    let hours_studied = vec![1.2, 2.5, 3.0, 4.0, 1.5, 2.0, 3.5, 4.5];
    let pass_fail_labels = vec![0.0, 0.0, 1.0, 1.0, 0.0, 0.0, 1.0, 1.0];

    let features: Vec<Vec<f64>> = hours_studied.iter().map(|&x| vec![x]).collect();

    let num_features = features[0].len();
    let mut model = LogisticRegression::new(num_features);

    let alpha = 0.01;
    let num_iterations = 1000;

    model.train(&features, &pass_fail_labels, alpha, num_iterations);

    let example_hours_studied = vec![2.8];
    let prediction = model.predict(&example_hours_studied);
    let guess = model.guess(&example_hours_studied);
    let cost = model.compute_cost(&features, &pass_fail_labels);

    println!("Prediction: {:.2}", prediction);
    println!("Result: {}", guess);
    println!("Final cost on training data: {:.4}", cost);

}