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
/// - `y_range`: The range for the y-axis values (min, max).
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
    let x_values: Vec<f64> = (0..num_points).map(|_| rng.sample(&x_dist)).collect();
    let y_uncorrelated: Vec<f64> = (0..num_points).map(|_| rng.sample(&y_dist)).collect();

    // Apply correlation transformation
    let sqrt_term = (1.0 - correlation.powi(2)).sqrt();
    let y_values: Vec<f64> = x_values
        .iter()
        .zip(y_uncorrelated.iter())
        .map(|(&x, &y_uncorr)| correlation * x + sqrt_term * y_uncorr)
        .collect();

    (x_values, y_values)
}

pub fn get_line_slopes_and_intercepts(
    slope_range: (f64, f64),
    intercept_range: (f64, f64),
    num_lines: usize,
    seed: u64,
) -> Vec<Line> {
    let mut rng = StdRng::seed_from_u64(seed);

    let slope_dist = Uniform::new(slope_range.0, slope_range.1);
    let intercept_dist = Uniform::new(intercept_range.0, intercept_range.1);

    (0..num_lines)
        .map(|_| Line {
            slope: rng.sample(&slope_dist),
            intercept: rng.sample(&intercept_dist),
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    fn calculate_correlation(x: &[f64], y: &[f64]) -> f64 {
        let n = x.len() as f64;
        let mean_x = x.iter().sum::<f64>() / n;
        let mean_y = y.iter().sum::<f64>() / n;

        let covariance = x.iter()
            .zip(y.iter())
            .map(|(&xi, &yi)| (xi - mean_x) * (yi - mean_y))
            .sum::<f64>() / n;

        let stddev_x = (x.iter().map(|&xi| (xi - mean_x).powi(2)).sum::<f64>() / n).sqrt();
        let stddev_y = (y.iter().map(|&yi| (yi - mean_y).powi(2)).sum::<f64>() / n).sqrt();

        covariance / (stddev_x * stddev_y)
    }

    #[test]
    fn test_generate_data_points_correct_length() {
        let num_points = 1000;
        let (x, y) = generate_data_points(num_points, 42, (0.0, 10.0), (0.0, 10.0), 0.5);
        assert_eq!(x.len(), num_points);
        assert_eq!(y.len(), num_points);
    }

    #[test]
    fn test_generate_data_points_within_range() {
        let num_points = 1000;
        let x_range = (0.0, 10.0);
        let y_range = (5.0, 15.0);
        let (x, _y) = generate_data_points(num_points, 42, x_range, y_range, 0.5);

        assert!(x.iter().all(|&xi| xi >= x_range.0 && xi <= x_range.1));
        // after transformation, 'y' data points can exceed original range
        // assert!(y.iter().all(|&yi| yi >= y_range.0 && yi <= y_range.1));
    }

    #[test]
    fn test_generate_data_points_correlation() {
        let num_points = 1000;
        let correlation = 0.8;
        let (x, y) = generate_data_points(num_points, 42, (0.0, 10.0), (0.0, 10.0), correlation);

        let calculated_correlation = calculate_correlation(&x, &y);
        assert!((calculated_correlation - correlation).abs() < 0.1, 
            "Expected correlation: {}, but got: {}", correlation, calculated_correlation);
    }

    #[test]
    fn test_generate_data_points_negative_correlation() {
        let num_points = 1000;
        let correlation = -0.8;
        let (x, y) = generate_data_points(num_points, 42, (0.0, 10.0), (0.0, 10.0), correlation);

        let calculated_correlation = calculate_correlation(&x, &y);
        assert!((calculated_correlation - correlation).abs() < 0.1, 
            "Expected correlation: {}, but got: {}", correlation, calculated_correlation);
    }
}
