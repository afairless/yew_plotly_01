use rand::prelude::*;
use rand::distributions::Uniform;

pub struct Line {
    pub slope: f64,
    pub intercept: f64,
}

/// Generates random data points for a scatterplot.
///
/// # Parameters
/// - `num_points`: The number of data points to generate.
/// - `seed`: The random seed for reproducibility.
/// - `x_range`: The range for the x-axis values (min, max).
/// - `y_mean`: The mean for the y-axis values (Normal distribution).
/// - `y_stddev`: The standard deviation for the y-axis values (Normal distribution).
pub fn generate_data_points(
    num_points: usize,
    seed: u64,
    x_range: (f64, f64),
    y_range: (f64, f64),
    correlation: f64,
) -> (Vec<f64>, Vec<f64>) {

    assert!((-1.0..=1.0).contains(&correlation), "Correlation must be between -1 and 1");

    let mut rng = StdRng::seed_from_u64(seed);

    // Uniform distributions for x and y
    let x_dist = Uniform::new(x_range.0, x_range.1);
    let y_dist = Uniform::new(y_range.0, y_range.1);

    // Generate uncorrelated x and y values
    let mut x_values: Vec<f64> = (0..num_points).map(|_| rng.sample(&x_dist)).collect();
    let mut y_uncorrelated: Vec<f64> = (0..num_points).map(|_| rng.sample(&y_dist)).collect();

    // Apply correlation transformation
    let sqrt_term = (1.0 - correlation.powi(2)).sqrt();
    let y_values: Vec<f64> = x_values
        .iter()
        .zip(y_uncorrelated.iter())
        .map(|(&x, &y_uncorr)| correlation * x + sqrt_term * y_uncorr)
        .collect();

    (x_values, y_values)

    // Generate data points
    //(0..num_points)
    //    .map(|_| {
    //        let x = rng.sample(x_dist);
    //        let y = rng.sample(y_dist);
    //        (x, y)
    //    })
    //    .unzip()
}

pub fn get_line_slopes_and_intercepts() -> Vec<Line> {
   vec![
       Line { slope: 1.0, intercept: 0.0, },
       Line { slope: 0.5, intercept: 2.0, },
       Line { slope: -0.5, intercept: 5.0, },
       Line { slope: 1.5, intercept: 1.0, },
   ]
}
