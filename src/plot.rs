use plotly::{Plot, Scatter};
use crate::utils::data::{generate_data_points, get_line_slopes_and_intercepts};


fn color_to_rgb(color: &str) -> (u8, u8, u8) {
    match color {
        "gray" => (128, 128, 128),
        "orange" => (255, 165, 0),
        _ => (0, 0, 0), // Default to black
    }
}

pub fn create_plot() -> (Plot, Vec<Box<Scatter<f64, f64>>>) {

    // Generate data points
    let x_min = 0.0;
    let x_max = 10.0;
    let (x_values, y_values) = generate_data_points(
        100, 42, (x_min, x_max), 5.0, 2.0);

    // Create a scatter trace with the generated data
    let trace = Scatter::new(x_values, y_values)
        .mode(plotly::common::Mode::Markers)
        .name("Data Points");

    let mut plot = Plot::new();
    plot.add_trace(trace);

    // Disable the legend
    plot.set_layout(plotly::Layout::new().show_legend(false).auto_size(true));

    let x_line = vec![x_min, x_max];
    let lines = get_line_slopes_and_intercepts();

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
