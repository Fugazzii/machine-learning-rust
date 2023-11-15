pub struct LogisticRegression {
    weights: Vec<f64>,
    bias: f64
}

impl LogisticRegression {
    
    pub fn new(num_features: usize) -> Self {
        let weights = vec![0.0; num_features + 1];
        LogisticRegression { weights, bias: 0.0 }
    }

    pub fn train(&mut self, features: &Vec<Vec<f64>>, labels: &Vec<f64>, alpha: f64, num_iterations: usize) {
        let m = features.len() as f64;

        for _ in 0..num_iterations {
            let mut gradient_weights = vec![0.0; self.weights.len()];
            let mut gradient_bias = 0.0;

            for (feature_set, &label) in features.iter().zip(labels) {
                let prediction = self.predict(feature_set);

                for (grad, &feature) in gradient_weights.iter_mut().zip(feature_set) {
                    *grad += (prediction - label) * feature;
                }
                gradient_bias += prediction - label;
            }

            for (weight, grad) in self.weights.iter_mut().zip(gradient_weights) {
                *weight -= alpha * grad / m;
            }
            self.bias -= alpha * gradient_bias / m;
        }
    }

    pub fn predict(&self, features: &Vec<f64>) -> f64 {
        let z: f64 = self.weights
            .iter()
            .zip(features.iter())
            .map(|(&w, &x)| w * x)
            .sum();
        self.sigmoid(z)
    }

    pub fn guess(&self, features: &Vec<f64>) -> bool {
        let z: f64 = self.weights
            .iter()
            .zip(features.iter())
            .map(|(&w, &x)| w * x)
            .sum();
        self.sigmoid(z) > 0.5
    }

    pub fn compute_cost(&self, features: &Vec<Vec<f64>>, labels: &Vec<f64>) -> f64 {
        let m = features.len() as f64;
        let mut cost = 0.0;

        for (feature_set, &label) in features.iter().zip(labels) {
            let prediction = self.predict(feature_set);
            cost += -label * prediction.ln() - (1.0 - label) * (1.0 - prediction).ln();
        }

        cost / m
    }

    fn sigmoid(&self, z: f64) -> f64 {
        1.0 / (1.0 + (-z).exp())
    }

}