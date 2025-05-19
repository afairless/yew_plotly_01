use plotly::{Plot, Scatter};
use yew::prelude::*;
mod utils;
use utils::data::{generate_data_points, get_line_slopes_and_intercepts};

#[function_component(App)]
pub fn plot_component() -> Html {
    let p = yew_hooks::use_async::<_, _, ()>({
        let id = "plot-div";
        let mut plot = Plot::new();

        // Generate data points
        let x_min = 0.0;
        let x_max = 10.0;
        let data = generate_data_points(100, 42, (x_min, x_max), 5.0, 2.0);
        let (x_values, y_values): (Vec<f64>, Vec<f64>) = data.into_iter().unzip();

        // Create a scatter trace with the generated data
        let trace = Scatter::new(x_values, y_values)
            .mode(plotly::common::Mode::Markers); // Display points as dots
        plot.add_trace(trace);

        // Define lines
        let x_line = vec![x_min, x_max];
        let lines = get_line_slopes_and_intercepts();

        for (i, (m, b)) in lines.iter().enumerate() {
            // Calculate y-values for the line
            let y_line = vec![m * x_min + b, m * x_max + b];

            // Set color and alpha based on the line index
            let color = if i < lines.len() - 1 {
                "rgba(128, 128, 128, 0.3)" // Gray with alpha 0.3
            } else {
                "rgba(255, 165, 0, 0.8)" // Orange with alpha 0.8
            };

            // Create a Scatter trace for the line
            let line_trace = Scatter::new(x_line.clone(), y_line)
                .mode(plotly::common::Mode::Lines)
                .name(format!("Line {}", i + 1)) // Optional: Add a name for the legend
                .line(plotly::common::Line::new().color(color));

            plot.add_trace(line_trace);
        }


        let layout = plotly::Layout::new()
            .title("Displaying a Chart in Yew")
            .show_legend(false); // Disable the legend
        plot.set_layout(layout);

        async move {
            plotly::bindings::new_plot(id, &plot).await;
            Ok(())
        }
    });
    // Only on first render
    use_effect_with((), move |_| {
        p.run();
    });

    html! {
        <div id="plot-div"></div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
