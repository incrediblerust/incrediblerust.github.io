---
title: "Machine Learning with Rust"
difficulty: advanced
version: "1.85.0"
---

# Machine Learning with Rust

Rust's performance and safety make it excellent for machine learning applications. Let's explore ML fundamentals and build practical models from scratch.

## Linear Algebra Foundations

### Matrix Operations

```rust
use std::ops::{Add, Mul};

#[derive(Debug, Clone)]
pub struct Matrix {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }
    
    pub fn from_vec(rows: usize, cols: usize, data: Vec<f64>) -> Result<Self, &'static str> {
        if data.len() != rows * cols {
            return Err("Data length doesn't match matrix dimensions");
        }
        
        Ok(Self { rows, cols, data })
    }
    
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self::new(rows, cols)
    }
    
    pub fn ones(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![1.0; rows * cols],
        }
    }
    
    pub fn identity(size: usize) -> Self {
        let mut matrix = Self::zeros(size, size);
        for i in 0..size {
            matrix.set(i, i, 1.0);
        }
        matrix
    }
    
    pub fn random(rows: usize, cols: usize) -> Self {
        use rand::prelude::*;
        let mut rng = thread_rng();
        let data: Vec<f64> = (0..rows * cols)
            .map(|_| rng.gen_range(-1.0..1.0))
            .collect();
        
        Self { rows, cols, data }
    }
    
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }
    
    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row * self.cols + col] = value;
    }
    
    pub fn transpose(&self) -> Matrix {
        let mut result = Matrix::zeros(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols {
                result.set(j, i, self.get(i, j));
            }
        }
        result
    }
    
    pub fn dot(&self, other: &Matrix) -> Result<Matrix, &'static str> {
        if self.cols != other.rows {
            return Err("Matrix dimensions don't match for multiplication");
        }
        
        let mut result = Matrix::zeros(self.rows, other.cols);
        
        for i in 0..self.rows {
            for j in 0..other.cols {
                let mut sum = 0.0;
                for k in 0..self.cols {
                    sum += self.get(i, k) * other.get(k, j);
                }
                result.set(i, j, sum);
            }
        }
        
        Ok(result)
    }
    
    pub fn map<F>(&self, f: F) -> Matrix
    where
        F: Fn(f64) -> f64,
    {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.data.iter().map(|&x| f(x)).collect(),
        }
    }
    
    pub fn element_wise_mul(&self, other: &Matrix) -> Result<Matrix, &'static str> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err("Matrix dimensions must match for element-wise multiplication");
        }
        
        let data: Vec<f64> = self.data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a * b)
            .collect();
        
        Ok(Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        })
    }
    
    pub fn sum(&self) -> f64 {
        self.data.iter().sum()
    }
    
    pub fn mean(&self) -> f64 {
        self.sum() / (self.rows * self.cols) as f64
    }
}

impl Add for &Matrix {
    type Output = Matrix;
    
    fn add(self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        
        let data: Vec<f64> = self.data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a + b)
            .collect();
        
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data,
        }
    }
}

// Activation Functions
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

pub fn sigmoid_derivative(x: f64) -> f64 {
    let s = sigmoid(x);
    s * (1.0 - s)
}

pub fn relu(x: f64) -> f64 {
    x.max(0.0)
}

pub fn relu_derivative(x: f64) -> f64 {
    if x > 0.0 { 1.0 } else { 0.0 }
}

pub fn tanh_activation(x: f64) -> f64 {
    x.tanh()
}

pub fn tanh_derivative(x: f64) -> f64 {
    1.0 - x.tanh().powi(2)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test matrix operations
    let a = Matrix::from_vec(2, 3, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0])?;
    let b = Matrix::from_vec(3, 2, vec![7.0, 8.0, 9.0, 10.0, 11.0, 12.0])?;
    
    println!("Matrix A: {:?}", a);
    println!("Matrix B: {:?}", b);
    
    let c = a.dot(&b)?;
    println!("A * B: {:?}", c);
    
    let sigmoid_matrix = a.map(sigmoid);
    println!("Sigmoid(A): {:?}", sigmoid_matrix);
    
    Ok(())
}
```

## Linear Regression

```rust
#[derive(Debug)]
pub struct LinearRegression {
    pub weights: Matrix,
    pub bias: f64,
    pub learning_rate: f64,
}

impl LinearRegression {
    pub fn new(features: usize, learning_rate: f64) -> Self {
        Self {
            weights: Matrix::random(features, 1),
            bias: 0.0,
            learning_rate,
        }
    }
    
    pub fn predict(&self, x: &Matrix) -> Result<Matrix, &'static str> {
        let mut predictions = x.dot(&self.weights)?;
        
        // Add bias
        for i in 0..predictions.rows {
            let current = predictions.get(i, 0);
            predictions.set(i, 0, current + self.bias);
        }
        
        Ok(predictions)
    }
    
    pub fn train(&mut self, x: &Matrix, y: &Matrix, epochs: usize) -> Result<Vec<f64>, &'static str> {
        let mut losses = Vec::new();
        let m = x.rows as f64;
        
        for epoch in 0..epochs {
            // Forward pass
            let predictions = self.predict(x)?;
            
            // Calculate loss (Mean Squared Error)
            let mut loss = 0.0;
            for i in 0..y.rows {
                let diff = predictions.get(i, 0) - y.get(i, 0);
                loss += diff * diff;
            }
            loss /= m;
            losses.push(loss);
            
            // Calculate gradients
            let mut dw = Matrix::zeros(self.weights.rows, 1);
            let mut db = 0.0;
            
            for i in 0..x.rows {
                let error = predictions.get(i, 0) - y.get(i, 0);
                db += error;
                
                for j in 0..x.cols {
                    let current_dw = dw.get(j, 0);
                    dw.set(j, 0, current_dw + error * x.get(i, j));
                }
            }
            
            // Update weights and bias
            for i in 0..self.weights.rows {
                let current_weight = self.weights.get(i, 0);
                let gradient = dw.get(i, 0) / m;
                self.weights.set(i, 0, current_weight - self.learning_rate * gradient);
            }
            
            self.bias -= self.learning_rate * (db / m);
            
            if epoch % 100 == 0 {
                println!("Epoch {}: Loss = {:.6}", epoch, loss);
            }
        }
        
        Ok(losses)
    }
    
    pub fn r_squared(&self, x: &Matrix, y: &Matrix) -> Result<f64, &'static str> {
        let predictions = self.predict(x)?;
        let y_mean = y.mean();
        
        let mut ss_res = 0.0; // Sum of squares of residuals
        let mut ss_tot = 0.0; // Total sum of squares
        
        for i in 0..y.rows {
            let y_true = y.get(i, 0);
            let y_pred = predictions.get(i, 0);
            
            ss_res += (y_true - y_pred).powi(2);
            ss_tot += (y_true - y_mean).powi(2);
        }
        
        Ok(1.0 - (ss_res / ss_tot))
    }
}

// Example usage
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate sample data: y = 2x + 1 + noise
    use rand::prelude::*;
    let mut rng = thread_rng();
    
    let samples = 100;
    let mut x_data = Vec::new();
    let mut y_data = Vec::new();
    
    for _ in 0..samples {
        let x = rng.gen_range(0.0..10.0);
        let noise = rng.gen_range(-0.5..0.5);
        let y = 2.0 * x + 1.0 + noise;
        
        x_data.push(x);
        y_data.push(y);
    }
    
    let x = Matrix::from_vec(samples, 1, x_data)?;
    let y = Matrix::from_vec(samples, 1, y_data)?;
    
    // Train model
    let mut model = LinearRegression::new(1, 0.01);
    println!("Training linear regression...");
    
    let losses = model.train(&x, &y, 1000)?;
    
    println!("Final weights: {:?}", model.weights);
    println!("Final bias: {:.6}", model.bias);
    
    // Calculate R-squared
    let r2 = model.r_squared(&x, &y)?;
    println!("R-squared: {:.6}", r2);
    
    // Make predictions
    let test_x = Matrix::from_vec(3, 1, vec![5.0, 7.5, 10.0])?;
    let predictions = model.predict(&test_x)?;
    
    println!("Predictions:");
    for i in 0..test_x.rows {
        println!("x = {:.1}, predicted y = {:.3}", test_x.get(i, 0), predictions.get(i, 0));
    }
    
    Ok(())
}
```

## Neural Network from Scratch

```rust
#[derive(Debug, Clone)]
pub struct Layer {
    pub weights: Matrix,
    pub biases: Matrix,
    pub activations: Matrix,
    pub z_values: Matrix,
}

impl Layer {
    pub fn new(input_size: usize, output_size: usize) -> Self {
        Self {
            weights: Matrix::random(input_size, output_size),
            biases: Matrix::zeros(1, output_size),
            activations: Matrix::zeros(1, output_size),
            z_values: Matrix::zeros(1, output_size),
        }
    }
}

#[derive(Debug)]
pub struct NeuralNetwork {
    pub layers: Vec<Layer>,
    pub learning_rate: f64,
}

impl NeuralNetwork {
    pub fn new(layer_sizes: Vec<usize>, learning_rate: f64) -> Self {
        let mut layers = Vec::new();
        
        for i in 0..layer_sizes.len() - 1 {
            layers.push(Layer::new(layer_sizes[i], layer_sizes[i + 1]));
        }
        
        Self {
            layers,
            learning_rate,
        }
    }
    
    pub fn forward(&mut self, input: &Matrix) -> Result<Matrix, &'static str> {
        let mut current_input = input.clone();
        
        for layer in &mut self.layers {
            // Z = X * W + B
            layer.z_values = current_input.dot(&layer.weights)?;
            
            // Add bias
            for i in 0..layer.z_values.rows {
                for j in 0..layer.z_values.cols {
                    let z = layer.z_values.get(i, j);
                    let bias = layer.biases.get(0, j);
                    layer.z_values.set(i, j, z + bias);
                }
            }
            
            // Apply activation function (sigmoid)
            layer.activations = layer.z_values.map(sigmoid);
            current_input = layer.activations.clone();
        }
        
        Ok(current_input)
    }
    
    pub fn backward(&mut self, input: &Matrix, target: &Matrix) -> Result<f64, &'static str> {
        // Forward pass first
        let output = self.forward(input)?;
        
        // Calculate loss (Mean Squared Error)
        let mut loss = 0.0;
        for i in 0..output.rows {
            for j in 0..output.cols {
                let diff = output.get(i, j) - target.get(i, j);
                loss += diff * diff;
            }
        }
        loss /= (output.rows * output.cols) as f64;
        
        // Backward pass
        let mut delta = Matrix::zeros(output.rows, output.cols);
        
        // Output layer error
        for i in 0..output.rows {
            for j in 0..output.cols {
                let output_val = output.get(i, j);
                let target_val = target.get(i, j);
                let sigmoid_deriv = sigmoid_derivative(self.layers.last().unwrap().z_values.get(i, j));
                delta.set(i, j, (output_val - target_val) * sigmoid_deriv);
            }
        }
        
        // Backpropagate through layers
        for layer_idx in (0..self.layers.len()).rev() {
            let current_input = if layer_idx == 0 {
                input.clone()
            } else {
                self.layers[layer_idx - 1].activations.clone()
            };
            
            // Calculate weight gradients
            let weight_gradients = current_input.transpose().dot(&delta)?;
            
            // Calculate bias gradients
            let mut bias_gradients = Matrix::zeros(1, delta.cols);
            for j in 0..delta.cols {
                let mut sum = 0.0;
                for i in 0..delta.rows {
                    sum += delta.get(i, j);
                }
                bias_gradients.set(0, j, sum / delta.rows as f64);
            }
            
            // Update weights and biases
            for i in 0..self.layers[layer_idx].weights.rows {
                for j in 0..self.layers[layer_idx].weights.cols {
                    let current_weight = self.layers[layer_idx].weights.get(i, j);
                    let gradient = weight_gradients.get(i, j);
                    self.layers[layer_idx].weights.set(i, j, current_weight - self.learning_rate * gradient);
                }
            }
            
            for j in 0..self.layers[layer_idx].biases.cols {
                let current_bias = self.layers[layer_idx].biases.get(0, j);
                let gradient = bias_gradients.get(0, j);
                self.layers[layer_idx].biases.set(0, j, current_bias - self.learning_rate * gradient);
            }
            
            // Calculate delta for previous layer
            if layer_idx > 0 {
                let weights_t = self.layers[layer_idx].weights.transpose();
                let new_delta = delta.dot(&weights_t)?;
                
                delta = Matrix::zeros(new_delta.rows, new_delta.cols);
                for i in 0..new_delta.rows {
                    for j in 0..new_delta.cols {
                        let z_val = self.layers[layer_idx - 1].z_values.get(i, j);
                        let deriv = sigmoid_derivative(z_val);
                        delta.set(i, j, new_delta.get(i, j) * deriv);
                    }
                }
            }
        }
        
        Ok(loss)
    }
    
    pub fn train(&mut self, x: &Matrix, y: &Matrix, epochs: usize) -> Result<Vec<f64>, &'static str> {
        let mut losses = Vec::new();
        
        for epoch in 0..epochs {
            let loss = self.backward(x, y)?;
            losses.push(loss);
            
            if epoch % 100 == 0 {
                println!("Epoch {}: Loss = {:.6}", epoch, loss);
            }
        }
        
        Ok(losses)
    }
    
    pub fn predict(&mut self, input: &Matrix) -> Result<Matrix, &'static str> {
        self.forward(input)
    }
}

// XOR Problem Example
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // XOR dataset
    let x = Matrix::from_vec(4, 2, vec![
        0.0, 0.0,
        0.0, 1.0,
        1.0, 0.0,
        1.0, 1.0,
    ])?;
    
    let y = Matrix::from_vec(4, 1, vec![
        0.0,
        1.0,
        1.0,
        0.0,
    ])?;
    
    // Create neural network: 2 inputs, 4 hidden, 1 output
    let mut nn = NeuralNetwork::new(vec![2, 4, 1], 0.5);
    
    println!("Training neural network on XOR problem...");
    let losses = nn.train(&x, &y, 2000)?;
    
    // Test the network
    println!("\nTesting network:");
    let predictions = nn.predict(&x)?;
    
    for i in 0..x.rows {
        let input1 = x.get(i, 0);
        let input2 = x.get(i, 1);
        let expected = y.get(i, 0);
        let predicted = predictions.get(i, 0);
        
        println!(
            "Input: ({:.0}, {:.0}) | Expected: {:.0} | Predicted: {:.3}",
            input1, input2, expected, predicted
        );
    }
    
    Ok(())
}
```

## K-Means Clustering

```rust
use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Point {
    pub features: Vec<f64>,
    pub cluster: Option<usize>,
}

impl Point {
    pub fn new(features: Vec<f64>) -> Self {
        Self {
            features,
            cluster: None,
        }
    }
    
    pub fn distance(&self, other: &Point) -> f64 {
        self.features
            .iter()
            .zip(other.features.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

#[derive(Debug)]
pub struct KMeans {
    pub k: usize,
    pub centroids: Vec<Point>,
    pub max_iterations: usize,
    pub tolerance: f64,
}

impl KMeans {
    pub fn new(k: usize, max_iterations: usize, tolerance: f64) -> Self {
        Self {
            k,
            centroids: Vec::new(),
            max_iterations,
            tolerance,
        }
    }
    
    pub fn fit(&mut self, data: &mut Vec<Point>) -> Result<(), &'static str> {
        if data.is_empty() {
            return Err("No data provided");
        }
        
        let feature_count = data[0].features.len();
        
        // Initialize centroids randomly
        self.initialize_centroids(data, feature_count);
        
        for iteration in 0..self.max_iterations {
            let old_centroids = self.centroids.clone();
            
            // Assign points to nearest centroid
            self.assign_clusters(data);
            
            // Update centroids
            self.update_centroids(data, feature_count);
            
            // Check for convergence
            let max_movement = self.calculate_max_movement(&old_centroids);
            
            println!("Iteration {}: Max centroid movement = {:.6}", iteration + 1, max_movement);
            
            if max_movement < self.tolerance {
                println!("Converged after {} iterations", iteration + 1);
                break;
            }
        }
        
        Ok(())
    }
    
    fn initialize_centroids(&mut self, data: &[Point], feature_count: usize) {
        let mut rng = thread_rng();
        self.centroids.clear();
        
        // Use random points from data as initial centroids
        let mut indices: Vec<usize> = (0..data.len()).collect();
        indices.shuffle(&mut rng);
        
        for i in 0..self.k {
            if i < data.len() {
                self.centroids.push(data[indices[i]].clone());
            } else {
                // If we have fewer data points than clusters, create random centroids
                let mut features = Vec::new();
                for _ in 0..feature_count {
                    features.push(rng.gen_range(-1.0..1.0));
                }
                self.centroids.push(Point::new(features));
            }
        }
    }
    
    fn assign_clusters(&self, data: &mut Vec<Point>) {
        for point in data.iter_mut() {
            let mut min_distance = f64::INFINITY;
            let mut closest_cluster = 0;
            
            for (cluster_idx, centroid) in self.centroids.iter().enumerate() {
                let distance = point.distance(centroid);
                if distance < min_distance {
                    min_distance = distance;
                    closest_cluster = cluster_idx;
                }
            }
            
            point.cluster = Some(closest_cluster);
        }
    }
    
    fn update_centroids(&mut self, data: &[Point], feature_count: usize) {
        for cluster_idx in 0..self.k {
            let cluster_points: Vec<&Point> = data
                .iter()
                .filter(|p| p.cluster == Some(cluster_idx))
                .collect();
            
            if cluster_points.is_empty() {
                continue;
            }
            
            let mut new_features = vec![0.0; feature_count];
            
            for point in &cluster_points {
                for (i, feature) in point.features.iter().enumerate() {
                    new_features[i] += feature;
                }
            }
            
            for feature in &mut new_features {
                *feature /= cluster_points.len() as f64;
            }
            
            self.centroids[cluster_idx] = Point::new(new_features);
        }
    }
    
    fn calculate_max_movement(&self, old_centroids: &[Point]) -> f64 {
        self.centroids
            .iter()
            .zip(old_centroids.iter())
            .map(|(new, old)| new.distance(old))
            .fold(0.0, f64::max)
    }
    
    pub fn predict(&self, point: &Point) -> usize {
        let mut min_distance = f64::INFINITY;
        let mut closest_cluster = 0;
        
        for (cluster_idx, centroid) in self.centroids.iter().enumerate() {
            let distance = point.distance(centroid);
            if distance < min_distance {
                min_distance = distance;
                closest_cluster = cluster_idx;
            }
        }
        
        closest_cluster
    }
    
    pub fn calculate_inertia(&self, data: &[Point]) -> f64 {
        let mut inertia = 0.0;
        
        for point in data {
            if let Some(cluster_idx) = point.cluster {
                let distance = point.distance(&self.centroids[cluster_idx]);
                inertia += distance.powi(2);
            }
        }
        
        inertia
    }
}

// Generate sample data
fn generate_clusters(centers: Vec<Vec<f64>>, samples_per_cluster: usize, noise: f64) -> Vec<Point> {
    let mut rng = thread_rng();
    let mut data = Vec::new();
    
    for center in centers {
        for _ in 0..samples_per_cluster {
            let mut features = Vec::new();
            for &coord in &center {
                let noise_val = rng.gen_range(-noise..noise);
                features.push(coord + noise_val);
            }
            data.push(Point::new(features));
        }
    }
    
    data.shuffle(&mut rng);
    data
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate sample data with 3 clusters
    let cluster_centers = vec![
        vec![2.0, 2.0],
        vec![6.0, 6.0],
        vec![2.0, 6.0],
    ];
    
    let mut data = generate_clusters(cluster_centers, 30, 0.5);
    
    println!("Generated {} data points", data.len());
    
    // Perform K-means clustering
    let mut kmeans = KMeans::new(3, 100, 0.001);
    kmeans.fit(&mut data)?;
    
    // Print cluster centroids
    println!("\nFinal centroids:");
    for (i, centroid) in kmeans.centroids.iter().enumerate() {
        println!("Cluster {}: {:?}", i, centroid.features);
    }
    
    // Calculate and print inertia
    let inertia = kmeans.calculate_inertia(&data);
    println!("Inertia: {:.6}", inertia);
    
    // Count points in each cluster
    let mut cluster_counts = vec![0; kmeans.k];
    for point in &data {
        if let Some(cluster) = point.cluster {
            cluster_counts[cluster] += 1;
        }
    }
    
    println!("\nCluster sizes:");
    for (i, count) in cluster_counts.iter().enumerate() {
        println!("Cluster {}: {} points", i, count);
    }
    
    // Test prediction on new point
    let test_point = Point::new(vec![3.0, 3.0]);
    let predicted_cluster = kmeans.predict(&test_point);
    println!("\nTest point {:?} assigned to cluster {}", test_point.features, predicted_cluster);
    
    Ok(())
}
```

## Decision Tree

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Sample {
    pub features: Vec<f64>,
    pub label: String,
}

impl Sample {
    pub fn new(features: Vec<f64>, label: String) -> Self {
        Self { features, label }
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub feature_index: Option<usize>,
    pub threshold: Option<f64>,
    pub label: Option<String>,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new_leaf(label: String) -> Self {
        Self {
            feature_index: None,
            threshold: None,
            label: Some(label),
            left: None,
            right: None,
        }
    }
    
    pub fn new_split(feature_index: usize, threshold: f64) -> Self {
        Self {
            feature_index: Some(feature_index),
            threshold: Some(threshold),
            label: None,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
pub struct DecisionTree {
    pub root: Option<Node>,
    pub max_depth: usize,
    pub min_samples_split: usize,
}

impl DecisionTree {
    pub fn new(max_depth: usize, min_samples_split: usize) -> Self {
        Self {
            root: None,
            max_depth,
            min_samples_split,
        }
    }
    
    pub fn fit(&mut self, data: &[Sample]) {
        self.root = Some(self.build_tree(data, 0));
    }
    
    fn build_tree(&self, data: &[Sample], depth: usize) -> Node {
        // Check stopping criteria
        if depth >= self.max_depth 
            || data.len() < self.min_samples_split 
            || self.is_pure(data) {
            return Node::new_leaf(self.majority_class(data));
        }
        
        // Find best split
        let (best_feature, best_threshold) = self.find_best_split(data);
        
        // Split data
        let (left_data, right_data) = self.split_data(data, best_feature, best_threshold);
        
        // Create split node
        let mut node = Node::new_split(best_feature, best_threshold);
        
        // Recursively build subtrees
        if !left_data.is_empty() {
            node.left = Some(Box::new(self.build_tree(&left_data, depth + 1)));
        }
        
        if !right_data.is_empty() {
            node.right = Some(Box::new(self.build_tree(&right_data, depth + 1)));
        }
        
        node
    }
    
    fn is_pure(&self, data: &[Sample]) -> bool {
        if data.is_empty() {
            return true;
        }
        
        let first_label = &data[0].label;
        data.iter().all(|sample| sample.label == *first_label)
    }
    
    fn majority_class(&self, data: &[Sample]) -> String {
        let mut counts = HashMap::new();
        
        for sample in data {
            *counts.entry(sample.label.clone()).or_insert(0) += 1;
        }
        
        counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(label, _)| label)
            .unwrap_or_else(|| "unknown".to_string())
    }
    
    fn find_best_split(&self, data: &[Sample]) -> (usize, f64) {
        let mut best_gini = f64::INFINITY;
        let mut best_feature = 0;
        let mut best_threshold = 0.0;
        
        if data.is_empty() {
            return (best_feature, best_threshold);
        }
        
        let feature_count = data[0].features.len();
        
        for feature_index in 0..feature_count {
            // Get unique values for this feature
            let mut values: Vec<f64> = data
                .iter()
                .map(|sample| sample.features[feature_index])
                .collect();
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            values.dedup();
            
            // Try each threshold
            for i in 0..values.len().saturating_sub(1) {
                let threshold = (values[i] + values[i + 1]) / 2.0;
                let gini = self.calculate_gini_split(data, feature_index, threshold);
                
                if gini < best_gini {
                    best_gini = gini;
                    best_feature = feature_index;
                    best_threshold = threshold;
                }
            }
        }
        
        (best_feature, best_threshold)
    }
    
    fn calculate_gini_split(&self, data: &[Sample], feature_index: usize, threshold: f64) -> f64 {
        let (left_data, right_data) = self.split_data(data, feature_index, threshold);
        
        let total_samples = data.len() as f64;
        let left_weight = left_data.len() as f64 / total_samples;
        let right_weight = right_data.len() as f64 / total_samples;
        
        left_weight * self.gini_impurity(&left_data) + right_weight * self.gini_impurity(&right_data)
    }
    
    fn gini_impurity(&self, data: &[Sample]) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        
        let mut counts = HashMap::new();
        for sample in data {
            *counts.entry(sample.label.clone()).or_insert(0) += 1;
        }
        
        let total = data.len() as f64;
        let mut gini = 1.0;
        
        for count in counts.values() {
            let probability = *count as f64 / total;
            gini -= probability * probability;
        }
        
        gini
    }
    
    fn split_data(&self, data: &[Sample], feature_index: usize, threshold: f64) -> (Vec<Sample>, Vec<Sample>) {
        let mut left = Vec::new();
        let mut right = Vec::new();
        
        for sample in data {
            if sample.features[feature_index] <= threshold {
                left.push(sample.clone());
            } else {
                right.push(sample.clone());
            }
        }
        
        (left, right)
    }
    
    pub fn predict(&self, features: &[f64]) -> Option<String> {
        self.predict_node(&self.root, features)
    }
    
    fn predict_node(&self, node: &Option<Node>, features: &[f64]) -> Option<String> {
        match node {
            Some(node) => {
                if let Some(ref label) = node.label {
                    // Leaf node
                    Some(label.clone())
                } else if let (Some(feature_index), Some(threshold)) = (node.feature_index, node.threshold) {
                    // Split node
                    if features[feature_index] <= threshold {
                        self.predict_node(&node.left, features)
                    } else {
                        self.predict_node(&node.right, features)
                    }
                } else {
                    None
                }
            }
            None => None,
        }
    }
    
    pub fn accuracy(&self, test_data: &[Sample]) -> f64 {
        if test_data.is_empty() {
            return 0.0;
        }
        
        let mut correct = 0;
        
        for sample in test_data {
            if let Some(predicted) = self.predict(&sample.features) {
                if predicted == sample.label {
                    correct += 1;
                }
            }
        }
        
        correct as f64 / test_data.len() as f64
    }
}

// Generate sample dataset (Iris-like)
fn generate_iris_data() -> Vec<Sample> {
    use rand::prelude::*;
    let mut rng = thread_rng();
    let mut data = Vec::new();
    
    // Setosa (small flowers)
    for _ in 0..50 {
        let features = vec![
            rng.gen_range(4.0..6.0),  // sepal length
            rng.gen_range(2.5..4.5),  // sepal width
            rng.gen_range(1.0..2.0),  // petal length
            rng.gen_range(0.1..0.5),  // petal width
        ];
        data.push(Sample::new(features, "setosa".to_string()));
    }
    
    // Versicolor (medium flowers)
    for _ in 0..50 {
        let features = vec![
            rng.gen_range(5.5..7.0),
            rng.gen_range(2.0..3.5),
            rng.gen_range(3.0..5.0),
            rng.gen_range(1.0..1.8),
        ];
        data.push(Sample::new(features, "versicolor".to_string()));
    }
    
    // Virginica (large flowers)
    for _ in 0..50 {
        let features = vec![
            rng.gen_range(6.0..8.0),
            rng.gen_range(2.5..4.0),
            rng.gen_range(4.5..7.0),
            rng.gen_range(1.5..2.5),
        ];
        data.push(Sample::new(features, "virginica".to_string()));
    }
    
    data.shuffle(&mut rng);
    data
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Generate dataset
    let data = generate_iris_data();
    
    // Split into train and test
    let split_index = (data.len() * 8) / 10; // 80% train, 20% test
    let train_data = &data[..split_index];
    let test_data = &data[split_index..];
    
    println!("Training samples: {}", train_data.len());
    println!("Test samples: {}", test_data.len());
    
    // Train decision tree
    let mut tree = DecisionTree::new(5, 5); // max_depth=5, min_samples_split=5
    tree.fit(train_data);
    
    // Test accuracy
    let accuracy = tree.accuracy(test_data);
    println!("Test accuracy: {:.2}%", accuracy * 100.0);
    
    // Make some predictions
    println!("\nSample predictions:");
    for (i, sample) in test_data.iter().take(5).enumerate() {
        if let Some(prediction) = tree.predict(&sample.features) {
            println!(
                "Sample {}: Features: {:?} | True: {} | Predicted: {}",
                i + 1, sample.features, sample.label, prediction
            );
        }
    }
    
    Ok(())
}
```

## Gradient Descent Variants

```rust
#[derive(Debug, Clone)]
pub struct SGD {
    pub learning_rate: f64,
}

impl SGD {
    pub fn new(learning_rate: f64) -> Self {
        Self { learning_rate }
    }
    
    pub fn update(&self, weights: &mut Matrix, gradients: &Matrix) {
        for i in 0..weights.rows {
            for j in 0..weights.cols {
                let current_weight = weights.get(i, j);
                let gradient = gradients.get(i, j);
                weights.set(i, j, current_weight - self.learning_rate * gradient);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Adam {
    pub learning_rate: f64,
    pub beta1: f64,
    pub beta2: f64,
    pub epsilon: f64,
    pub m: Matrix,
    pub v: Matrix,
    pub t: usize,
}

impl Adam {
    pub fn new(learning_rate: f64, shape: (usize, usize)) -> Self {
        Self {
            learning_rate,
            beta1: 0.9,
            beta2: 0.999,
            epsilon: 1e-8,
            m: Matrix::zeros(shape.0, shape.1),
            v: Matrix::zeros(shape.0, shape.1),
            t: 0,
        }
    }
    
    pub fn update(&mut self, weights: &mut Matrix, gradients: &Matrix) {
        self.t += 1;
        
        for i in 0..weights.rows {
            for j in 0..weights.cols {
                let gradient = gradients.get(i, j);
                
                // Update biased first moment estimate
                let m_val = self.beta1 * self.m.get(i, j) + (1.0 - self.beta1) * gradient;
                self.m.set(i, j, m_val);
                
                // Update biased second raw moment estimate
                let v_val = self.beta2 * self.v.get(i, j) + (1.0 - self.beta2) * gradient * gradient;
                self.v.set(i, j, v_val);
                
                // Compute bias-corrected first moment estimate
                let m_hat = m_val / (1.0 - self.beta1.powi(self.t as i32));
                
                // Compute bias-corrected second raw moment estimate
                let v_hat = v_val / (1.0 - self.beta2.powi(self.t as i32));
                
                // Update weights
                let current_weight = weights.get(i, j);
                let update = self.learning_rate * m_hat / (v_hat.sqrt() + self.epsilon);
                weights.set(i, j, current_weight - update);
            }
        }
    }
}

// Example: Comparing optimizers on a simple function
fn rosenbrock_function(x: f64, y: f64) -> f64 {
    let a = 1.0;
    let b = 100.0;
    (a - x).powi(2) + b * (y - x.powi(2)).powi(2)
}

fn rosenbrock_gradient(x: f64, y: f64) -> (f64, f64) {
    let a = 1.0;
    let b = 100.0;
    let dx = -2.0 * (a - x) - 4.0 * b * x * (y - x.powi(2));
    let dy = 2.0 * b * (y - x.powi(2));
    (dx, dy)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut x_sgd = -2.0;
    let mut y_sgd = 1.0;
    let sgd = SGD::new(0.001);
    
    let mut x_adam = -2.0;
    let mut y_adam = 1.0;
    let mut adam = Adam::new(0.01, (1, 1));
    
    println!("Optimizing Rosenbrock function:");
    println!("Target: (1.0, 1.0), Minimum: 0.0");
    println!();
    
    for iteration in 0..1000 {
        // SGD updates
        let (dx_sgd, dy_sgd) = rosenbrock_gradient(x_sgd, y_sgd);
        x_sgd -= sgd.learning_rate * dx_sgd;
        y_sgd -= sgd.learning_rate * dy_sgd;
        let loss_sgd = rosenbrock_function(x_sgd, y_sgd);
        
        // Adam updates
        let (dx_adam, dy_adam) = rosenbrock_gradient(x_adam, y_adam);
        let mut weights_adam = Matrix::from_vec(2, 1, vec![x_adam, y_adam])?;
        let gradients_adam = Matrix::from_vec(2, 1, vec![dx_adam, dy_adam])?;
        adam.update(&mut weights_adam, &gradients_adam);
        x_adam = weights_adam.get(0, 0);
        y_adam = weights_adam.get(1, 0);
        let loss_adam = rosenbrock_function(x_adam, y_adam);
        
        if iteration % 100 == 0 {
            println!("Iteration {}:", iteration);
            println!("  SGD: ({:.4}, {:.4}) Loss: {:.6}", x_sgd, y_sgd, loss_sgd);
            println!("  Adam: ({:.4}, {:.4}) Loss: {:.6}", x_adam, y_adam, loss_adam);
            println!();
        }
    }
    
    println!("Final results:");
    println!("SGD: ({:.4}, {:.4}) Loss: {:.6}", x_sgd, y_sgd, rosenbrock_function(x_sgd, y_sgd));
    println!("Adam: ({:.4}, {:.4}) Loss: {:.6}", x_adam, y_adam, rosenbrock_function(x_adam, y_adam));
    
    Ok(())
}
```

## Key ML Concepts

1. **Supervised Learning**: Learning from labeled data (regression, classification)
2. **Unsupervised Learning**: Finding patterns in unlabeled data (clustering, dimensionality reduction)
3. **Feature Engineering**: Transforming raw data into meaningful features
4. **Cross-validation**: Evaluating model performance on unseen data
5. **Regularization**: Preventing overfitting (L1, L2 regularization)
6. **Gradient Descent**: Optimization algorithm for finding minimum loss

## Best Practices

- Always normalize/standardize your data
- Split data into train/validation/test sets
- Use cross-validation for model selection
- Monitor for overfitting and underfitting
- Start with simple models before complex ones
- Use appropriate metrics for your problem type
- Implement proper error handling
- Profile and optimize performance-critical code

Machine learning in Rust provides excellent performance while maintaining memory safety. These implementations demonstrate the core concepts needed to build ML applications from scratch!