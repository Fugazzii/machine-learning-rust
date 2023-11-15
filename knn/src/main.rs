use knn::knn;

fn main() {
    let data: Vec<(f32, f32, String)> = vec![
        (1.319, 26.61, "Estonia".to_string()),
        (81.0, 852.0, "Turkey".to_string()),
        (2.9, 11.54, "Armenia".to_string()),
        (9.84, 40.75, "Azerbaijan".to_string()),
        (44.48, 112.0, "Ukraine".to_string()),
        (37.9, 526.0, "Poland".to_string())
    ];
    println!("{:?}", knn(3, &(4.0, 15.08, "Georgia".to_string()), &data));
}