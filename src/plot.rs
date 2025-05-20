use plotly::{Plot, Scatter};
use crate::utils::data::{generate_data_points, get_line_slopes_and_intercepts, Line};


fn color_to_rgb(color: &str) -> (u8, u8, u8) {
    match color {
        "gray" => (128, 128, 128),
        "orange" => (255, 165, 0),
        _ => (0, 0, 0), // Default to black
    }
}

pub fn generate_scatter_data(
    x_min: f64,
    x_max: f64,
) ->(Vec<f64>, Vec<f64>) {

    let num_points = 100;
    let seed = 2981328;
    let y_min = x_min;
    let y_max = x_max;
    let x_range = (x_min, x_max);
    let y_range = (y_min, y_max);
    let correlation = 0.5;

    generate_data_points(num_points, seed, x_range, y_range, correlation)
}

pub fn generate_lines() -> Vec<Line> {
    let slope_min = -10.0;
    let slope_max = 10.0;
    let intercept_min = -10.0;
    let intercept_max = 10.0;
    let num_lines = 5;
    let slope_range = (slope_min, slope_max);
    let intercept_range = (intercept_min, intercept_max);
    let seed = 604913;

    get_line_slopes_and_intercepts(slope_range, intercept_range, num_lines, seed)
}

pub fn create_scatterplot(
    x_min: f64,
    x_max: f64,
    x_values: Vec<f64>,
    y_values: Vec<f64>,
    lines: Vec<Line>,
) ->(Plot, Vec<Box<Scatter<f64, f64>>>) {

    // Create a scatter trace with the generated data
    let trace = Scatter::new(x_values, y_values)
        .mode(plotly::common::Mode::Markers)
        .name("Data Points");

    let mut plot = Plot::new();
    plot.add_trace(trace);

    // Disable the legend
    plot.set_layout(plotly::Layout::new().show_legend(false).auto_size(true));

    let x_line = vec![x_min, x_max];
    // let lines = generate_lines();

    let line_traces: Vec<Box<Scatter<f64, f64>>> = lines
        .iter()
        .enumerate()
        .map(|(i, line)| {

            let y_line = vec![
                line.slope * x_min + line.intercept,
                line.slope * x_max + line.intercept,
            ];

            let alpha = if i == lines.len() - 1 { 0.8 } else { 0.3 };
            let color = if i == lines.len() - 1 { "orange" } else { "gray" };
            let (r, g, b) = color_to_rgb(color); // Destructure the RGB tuple

            // let line_trace = Scatter::new(x_line.clone(), y_line)
            Scatter::new(x_line.clone(), y_line)
                .mode(plotly::common::Mode::Lines)
                .name(format!("Line {}", i + 1))
                .line(
                    plotly::common::Line::new()
                        .color(format!("rgba({}, {}, {}, {})", r, g, b, alpha))
                )
        })
        .collect();

    (plot, line_traces)

}
