use rand::prelude::*;
use rand::distributions::Uniform;
use rand_distr::Normal;

#[derive(PartialEq, Debug, Clone)]
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
    x_coef: f64,
    y_intercept: f64,
    correlation: f64,
) -> (Vec<f64>, Vec<f64>) {

    assert!((-1.0..=1.0).contains(&correlation), "Correlation must be between -1 and 1");

    let mut rng = StdRng::seed_from_u64(seed);

    let x_dist = Uniform::new(x_range.0, x_range.1);
    let x_std = (x_range.1 - x_range.0) / 12.0_f64.sqrt();

    let y_std = (x_coef * x_std * (1.0 - correlation.powi(2)).sqrt()) / correlation;

    let y_dist = Normal::new(0.0, y_std).unwrap();

    let noise: Vec<f64> = (0..num_points).map(|_| y_dist.sample(&mut rng)).collect();

    let x_values: Vec<f64> = (0..num_points).map(|_| rng.sample(&x_dist)).collect();
    let y_values: Vec<f64> = x_values.iter()
        .zip(noise.iter())
        .map(|(&xi, &ei)| x_coef * xi + y_intercept + ei)
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

pub fn calculate_mean_squared_error(
    x_values: &[f64], 
    y_values: &[f64], 
    line: &Line) -> f64 {

    assert_eq!(x_values.len(), y_values.len(), "x_values and y_values must have the same length");

    let n = x_values.len() as f64;

    let mse: f64 = x_values
        .iter()
        .zip(y_values.iter())
        .map(|(&x, &y)| {
            let predicted_y = line.slope * x + line.intercept;
            (y - predicted_y).powi(2)
        })
        .sum();

    mse / n
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
        let (x, y) = generate_data_points(num_points, 11355, (0.0, 10.0), 0.5, 0.0, 0.5);
        assert_eq!(x.len(), num_points);
        assert_eq!(y.len(), num_points);
    }

    #[test]
    fn test_generate_data_points_within_range() {
        let num_points = 1000;
        let x_range = (0.0, 10.0);
        let (x, _y) = generate_data_points(num_points, 11355, x_range, 0.5, 0.0, 0.5);

        assert!(x.iter().all(|&xi| xi >= x_range.0 && xi <= x_range.1));
        // after transformation, 'y' data points can exceed original range
        // assert!(y.iter().all(|&yi| yi >= y_range.0 && yi <= y_range.1));
    }

    #[test]
    fn test_generate_data_points_correlation() {
        let num_points = 1000;
        let correlation = 0.8;
        let (x, y) = generate_data_points(num_points, 11355, (0.0, 10.0), 0.5, 0.0, correlation);

        let calculated_correlation = calculate_correlation(&x, &y);
        assert!((calculated_correlation - correlation).abs() < 0.1, 
            "Expected correlation: {correlation}, but got: {calculated_correlation}");
    }

    #[test]
    fn test_generate_data_points_negative_correlation() {
        let num_points = 1000;
        let correlation = -0.8;
        let (x, y) = generate_data_points(num_points, 11355, (0.0, 10.0), -0.5, 0.0, correlation);

        let calculated_correlation = calculate_correlation(&x, &y);
        assert!((calculated_correlation - correlation).abs() < 0.1, 
            "Expected correlation: {correlation}, but got: {calculated_correlation}");
    }

    #[test]
    fn test_get_line_slopes_and_intercepts_correct_length() {
        let num_lines = 10;
        let lines = get_line_slopes_and_intercepts((0.0, 1.0), (0.0, 1.0), num_lines, 42);
        assert_eq!(lines.len(), num_lines);
    }

    #[test]
    fn test_get_line_slopes_and_intercepts_within_range() {
        let slope_range = (-1.0, 1.0);
        let intercept_range = (0.0, 5.0);
        let num_lines = 100;
        let lines = get_line_slopes_and_intercepts(slope_range, intercept_range, num_lines, 42);

        assert!(lines.iter().all(|line| line.slope >= slope_range.0 && line.slope <= slope_range.1));
        assert!(lines.iter().all(|line| line.intercept >= intercept_range.0 && line.intercept <= intercept_range.1));
    }

    #[test]
    fn test_get_line_slopes_and_intercepts_deterministic() {
        let slope_range = (0.0, 10.0);
        let intercept_range = (0.0, 10.0);
        let num_lines = 5;
        let seed = 42;

        let lines1 = get_line_slopes_and_intercepts(slope_range, intercept_range, num_lines, seed);
        let lines2 = get_line_slopes_and_intercepts(slope_range, intercept_range, num_lines, seed);

        assert_eq!(lines1, lines2, "The function should produce the same output for the same seed");
    }

    #[test]
    fn test_calculate_mean_squared_error_perfect_fit() {
        let x_values = vec![1.0, 2.0, 3.0];
        let y_values = vec![2.0, 4.0, 6.0];
        let line = Line { slope: 2.0, intercept: 0.0 };

        let mse = calculate_mean_squared_error(&x_values, &y_values, &line);
        assert_eq!(mse, 0.0, "MSE should be 0 for a perfect fit");
    }

    #[test]
    fn test_calculate_mean_squared_error_nonzero_error() {
        let x_values = vec![1.0, 2.0, 3.0];
        let y_values = vec![2.0, 4.0, 6.0];
        let line = Line { slope: 1.5, intercept: 0.5 };

        let mse = calculate_mean_squared_error(&x_values, &y_values, &line);
        assert!(mse > 0.0, "MSE should be greater than 0 for a non-perfect fit");
    }

    #[test]
    fn test_calculate_mean_squared_error_with_negative_slope() {
        let x_values = vec![1.0, 2.0, 3.0];
        let y_values = vec![2.0, 4.0, 6.0];
        let line = Line { slope: -1.0, intercept: 10.0 };

        let mse = calculate_mean_squared_error(&x_values, &y_values, &line);
        assert!(mse > 0.0, "MSE should be greater than 0 for a line with a negative slope");
    }

    #[test]
    fn test_calculate_mean_squared_error_empty_values() {
        let x_values: Vec<f64> = vec![];
        let y_values: Vec<f64> = vec![];
        let line = Line { slope: 1.0, intercept: 0.0 };

        let mse = calculate_mean_squared_error(&x_values, &y_values, &line);
        assert!(mse.is_nan(), "MSE should be NaN for empty input values");
    }

    #[test]
    #[should_panic(expected = "x_values and y_values must have the same length")]
    fn test_calculate_mean_squared_error_mismatched_lengths() {
        let x_values = vec![1.0, 2.0];
        let y_values = vec![2.0, 4.0, 6.0];
        let line = Line { slope: 1.0, intercept: 0.0 };

        calculate_mean_squared_error(&x_values, &y_values, &line);
    }
}
