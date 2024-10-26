// Cargo.toml
[package]
name = "gradient_descent"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
rand_chacha = "0.3"
ndarray = "0.15"
ndarray-rand = "0.14"

// src/main.rs
use ndarray::Array1;
use ndarray_rand::RandomExt;
use ndarray_rand::rand_distr::Uniform;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rand::Rng;

const LEARNING_RATE: f64 = 0.1;
const N_ITERATIONS: usize = 1000;
const N_POINTS: usize = 100;
const TRUE_B: f64 = 0.5;
const TRUE_W: f64 = -3.0;

/// Represents our linear regression model
struct LinearRegression {
    w: f64,
    b: f64,
}

impl LinearRegression {
    fn new(w: f64, b: f64) -> Self {
        Self { w, b }
    }

    fn predict(&self, x: &Array1<f64>) -> Array1<f64> {
        x * self.w + self.b
    }

    fn update_params(&mut self, w_grad: f64, b_grad: f64) {
        self.w -= LEARNING_RATE * w_grad;
        self.b -= LEARNING_RATE * b_grad;
    }
}

/// Generates synthetic data for training
fn generate_data(rng: &mut ChaCha8Rng) -> (Array1<f64>, Array1<f64>) {
    let x = Array1::random_using(N_POINTS, Uniform::new(0., 1.), rng);
    let noise = Array1::random_using(N_POINTS, Uniform::new(-0.1, 0.1), rng);
    let y = &x * TRUE_W + TRUE_B + noise;
    (x, y)
}

/// Computes mean squared error loss
fn compute_loss(y_pred: &Array1<f64>, y_true: &Array1<f64>) -> f64 {
    let error = y_pred - y_true;
    error.map(|e| e * e).mean().unwrap()
}

/// Computes gradients for parameters
fn compute_gradients(
    x: &Array1<f64>,
    error: &Array1<f64>,
) -> (f64, f64) {
    let w_grad = 2.0 * (x * error).mean().unwrap();
    let b_grad = 2.0 * error.mean().unwrap();
    (w_grad, b_grad)
}

/// Print simple ASCII visualization of the loss
fn print_loss_visualization(loss: f64, max_loss: f64, width: usize) {
    let normalized = (loss / max_loss * width as f64) as usize;
    print!("[");
    for i in 0..width {
        if i < normalized {
            print!("#");
        } else {
            print!(" ");
        }
    }
    println!("] {:.6}", loss);
}

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let (x_train, y_train) = generate_data(&mut rng);

    // Initialize model with random parameters
    let mut model = LinearRegression::new(
        rng.gen_range(-1.0..=1.0),
        rng.gen_range(-1.0..=1.0),
    );

    // Training loop
    let mut losses = Vec::with_capacity(N_ITERATIONS);
    let mut max_loss: f64 = 0.0;
    
    println!("\nStarting gradient descent training...");
    println!("True parameters: w = {:.4}, b = {:.4}", TRUE_W, TRUE_B);
    println!("Initial parameters: w = {:.4}, b = {:.4}", model.w, model.b);
    println!("\nTraining progress (loss visualization):");
    
    for iteration in 0..N_ITERATIONS {
        // Forward pass
        let y_pred = model.predict(&x_train);
        
        // Compute loss
        let loss = compute_loss(&y_pred, &y_train);
        losses.push(loss);
        max_loss = f64::max(max_loss, loss);
        
        // Compute gradients
        let error = &y_pred - &y_train;
        let (w_grad, b_grad) = compute_gradients(&x_train, &error);
        
        // Update parameters
        model.update_params(w_grad, b_grad);

        // Print progress every 100 iterations
        if iteration % 100 == 0 {
            println!("\nIteration {}:", iteration);
            print_loss_visualization(loss, max_loss, 50);
            println!("Parameters: w = {:.4}, b = {:.4}", model.w, model.b);
        }
    }

    println!("\nTraining completed!");
    println!("Final parameters: w = {:.4}, b = {:.4}", model.w, model.b);
    println!("True parameters: w = {:.4}, b = {:.4}", TRUE_W, TRUE_B);
    println!("Final loss: {:.6}", losses.last().unwrap());
    
    // Print some predictions vs actual values
    println!("\nSample predictions:");
    println!("   x   |  Actual y  | Predicted y");
    println!("--------------------------------");
    for i in 0..5 {
        let x = x_train[i];
        let y_true = y_train[i];
        let y_pred = model.predict(&Array1::from_vec(vec![x]))[0];
        println!("{:6.3} | {:10.3} | {:10.3}", x, y_true, y_pred);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_prediction() {
        let model = LinearRegression::new(2.0, 1.0);
        let x = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let expected = Array1::from_vec(vec![3.0, 5.0, 7.0]);
        let predicted = model.predict(&x);
        assert_eq!(predicted, expected);
    }

    #[test]
    fn test_loss_computation() {
        let pred = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let true_vals = Array1::from_vec(vec![1.0, 2.0, 3.0]);
        let loss = compute_loss(&pred, &true_vals);
        assert_eq!(loss, 0.0);
    }

    #[test]
    fn test_gradient_computation() {
        let x = Array1::from_vec(vec![1.0, 1.0, 1.0]);
        let error = Array1::from_vec(vec![1.0, 1.0, 1.0]);
        let (w_grad, b_grad) = compute_gradients(&x, &error);
        assert_eq!(w_grad, 2.0);
        assert_eq!(b_grad, 2.0);
    }
}