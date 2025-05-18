use plotly::{Plot, Scatter};
use yew::prelude::*;
mod utils; // Import the utils module
use utils::data::generate_data; // Import the generate_data function

#[function_component(App)]
pub fn plot_component() -> Html {
    let p = yew_hooks::use_async::<_, _, ()>({
        let id = "plot-div";
        let mut plot = Plot::new();

        // Generate data points
        let data = generate_data(100, 42, (0.0, 10.0), 5.0, 2.0);
        let (x_values, y_values): (Vec<f64>, Vec<f64>) = data.into_iter().unzip();

        // Create a scatter trace with the generated data
        // let trace = Scatter::new(x_values, y_values);
        let trace = Scatter::new(x_values, y_values)
            .mode(plotly::common::Mode::Markers); // Display points as dots
        plot.add_trace(trace);


        // Define the slope (m) and intercept (b)
        let x_min = 0.0;
        let x_max = 10.0;
        let m = 2.0; // slope
        let b = 1.0; // intercept

        // Use the x-range of your scatterplot to calculate the line points
        let x_line = vec![x_min, x_max];
        let y_line = vec![m * x_min + b, m * x_max + b];

        // Create a Scatter trace for the line
        let line_trace = Scatter::new(x_line, y_line)
            .mode(plotly::common::Mode::Lines)
            .name("Line");

        plot.add_trace(line_trace);


        let layout = plotly::Layout::new().title("Displaying a Chart in Yew");
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
