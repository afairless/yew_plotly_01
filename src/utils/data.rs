use rand::prelude::*;
use rand::distributions::Uniform;
use rand_distr::Normal;

/// Generates random data points for a scatterplot.
///
/// # Parameters
/// - `num_points`: The number of data points to generate.
/// - `seed`: The random seed for reproducibility.
/// - `x_range`: The range for the x-axis values (min, max).
/// - `y_mean`: The mean for the y-axis values (Normal distribution).
/// - `y_stddev`: The standard deviation for the y-axis values (Normal distribution).
///
/// # Returns
/// A vector of tuples representing the (x, y) coordinates of the data points.
pub fn generate_data_points(
    num_points: usize,
    seed: u64,
    x_range: (f64, f64),
    y_mean: f64,
    y_stddev: f64,
) -> Vec<(f64, f64)> {
    let mut rng = StdRng::seed_from_u64(seed);

    // Uniform distribution for x-axis
    let x_dist = Uniform::new(x_range.0, x_range.1);

    // Normal distribution for y-axis
    let y_dist = Normal::new(y_mean, y_stddev).unwrap();

    // Generate data points
    (0..num_points)
        .map(|_| {
            let x = rng.sample(x_dist);
            let y = rng.sample(y_dist);
            (x, y)
        })
        .collect()
}

/// Returns a vector of lines defined by their slope and intercept.
///
/// Each line is represented as a tuple `(slope, intercept)`.
pub fn get_line_slopes_and_intercepts() -> Vec<(f64, f64)> {
    vec![
        (1.0, 0.0),  // Line 1: slope = 1.0, intercept = 0.0
        (0.5, 2.0),  // Line 2: slope = 0.5, intercept = 2.0
        (-0.5, 5.0), // Line 3: slope = -0.5, intercept = 5.0
        (1.5, 1.0),  // Last line: slope = 2.0, intercept = 1.0
    ]
}
