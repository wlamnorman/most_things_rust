extern crate rand;
extern crate xgboost;

use rand::distributions::{Bernoulli, Uniform};
use rand::Rng;
use std::time::Instant;
use xgboost::{parameters, Booster, DMatrix};

fn main() {
    // Number of samples
    let n_train_samples = 10000;

    // Initialize random number generator
    let mut rng = rand::thread_rng();

    // Allocate space for data and target
    let mut x_train: Vec<f32> = Vec::with_capacity(n_train_samples * 14);
    let mut y_train: Vec<f32> = Vec::with_capacity(n_train_samples);

    for _ in 0..n_train_samples {
        // Generate binary variables
        let bernoulli_dist = Bernoulli::new(0.5).unwrap();
        for _ in 0..10 {
            x_train.push(if rng.sample(bernoulli_dist) { 1.0 } else { 0.0 });
        }

        // Generate discrete 0-40 variables
        let uniform_0_40 = Uniform::from(0..41);
        for _ in 0..2 {
            x_train.push(rng.sample(&uniform_0_40) as f32);
        }

        // Generate discrete 0-80000 variables
        let uniform_0_80000 = Uniform::from(0..80001);
        for _ in 0..2 {
            x_train.push(rng.sample(&uniform_0_80000) as f32);
        }

        // Generate target vector
        let uniform_200_800 = Uniform::from(200..801);
        y_train.push(rng.sample(&uniform_200_800) as f32);
    }

    // Convert to DMatrix format
    let mut dtrain = DMatrix::from_dense(&x_train, n_train_samples).unwrap();
    dtrain.set_labels(&y_train).unwrap();

    // Number of test samples
    let n_test_samples = 1;

    // Allocate space for data and target
    let mut x_test: Vec<f32> = Vec::with_capacity(n_test_samples * 14);
    let mut y_test: Vec<f32> = Vec::with_capacity(n_test_samples);

    for _ in 0..n_test_samples {
        // Generate binary variables
        let bernoulli_dist = Bernoulli::new(0.5).unwrap();
        for _ in 0..10 {
            x_test.push(if rng.sample(bernoulli_dist) { 1.0 } else { 0.0 });
        }

        // Generate discrete 0-40 variables
        let uniform_0_40 = Uniform::from(0..41);
        for _ in 0..2 {
            x_test.push(rng.sample(&uniform_0_40) as f32);
        }

        // Generate discrete 0-80000 variables
        let uniform_0_80000 = Uniform::from(0..80001);
        for _ in 0..2 {
            x_test.push(rng.sample(&uniform_0_80000) as f32);
        }

        // Generate target vector
        let uniform_200_800 = Uniform::from(200..801);
        y_test.push(rng.sample(&uniform_200_800) as f32);
    }

    // build overall training parameters
    let params = parameters::TrainingParametersBuilder::default()
        .dtrain(&dtrain)
        .build()
        .unwrap();

    // Convert to DMatrix format
    let mut dtest = DMatrix::from_dense(&x_test, n_test_samples).unwrap();
    dtest.set_labels(&y_test).unwrap();

    // specify datasets to evaluate against during training
    // let evaluation_sets = &[(&dtrain, "train"), (&dtest, "test")];

    // train model, and print evaluation data
    let bst = Booster::train(&params).unwrap();

    // Number of predictions you want to make
    let n_predictions = 50000 * 120;
    let start = Instant::now();
    for _ in 0..n_predictions {
        let _predictions = bst.predict(&dtest).unwrap();
    }
    let duration = start.elapsed();
    println!(
        "Time taken for {} predictions with XGBoost: {:?}",
        n_predictions, duration
    );

    let mut _i = 0;
    let start = Instant::now();
    for _ in 0..n_predictions {
        _i += 1;
    }
    let duration = start.elapsed();
    println!("Time taken for {} additions: {:?}", n_predictions, duration);
}
