use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;
use yew_hooks::use_async;
use std::rc::Rc;
use std::cell::RefCell;
use plotly::{Plot, Scatter};
mod utils;
mod plot;
use crate::plot::{generate_scatter_data, generate_lines};
use crate::utils::data::calculate_mean_squared_error;

#[function_component(App)]
pub fn plot_component() -> Html {

    let x_min = -10.0;
    let x_max = 10.0;
    let (x_values, y_values) = generate_scatter_data(x_min, x_max);
    let lines = generate_lines();

    let p1 = use_async::<_, _, ()>({
        let id = "plot1";
        let (plot, line_traces) = plot::create_scatterplot(
            x_min, x_max, x_values.clone(), y_values.clone(), lines.clone());
        // Wrap plot in Rc<RefCell<>> for shared ownership
        let plot = Rc::new(RefCell::new(plot)); 

        async move {
            // Borrow the inner Plot
            plotly::bindings::new_plot(id, &*plot.borrow()).await; 

            // Animate the lines
            for (i, trace) in line_traces.into_iter().enumerate() {
                let delay = i as u32 * 1000; // 1 second per animation step
                let window = window().unwrap();
                let id = id.to_string();
                let plot = Rc::clone(&plot); // Clone Rc for the closure
                let closure = Closure::wrap(Box::new(move || {
                    let id = id.clone();
                    let trace = trace.clone();
                    let plot = Rc::clone(&plot); // Clone Rc for plot
                    spawn_local(async move {
                        // Borrow mutably for add_trace
                        let mut plot = plot.borrow_mut(); 
                        plot.add_trace(Box::new(*trace));
                        plotly::bindings::react(&id, &plot).await;
                    });
                }) as Box<dyn Fn()>);

                window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        delay as i32,
                    )
                    .unwrap();

                closure.forget(); // Prevent memory leaks
            }

            Ok(())
        }
    });

    let p2 = use_async::<_, _, ()>({
        let id = "plot2";

        // Calculate the MSE for each line
        let mse_values: Vec<f64> = lines
            .iter()
            .map(|line| calculate_mean_squared_error(&x_values, &y_values, line))
            .collect();
        let indices: Vec<usize> = (0..mse_values.len()).collect();

        // Calculate axes limits
        let x_min = -0.3;
        let x_max_raw = indices.len() as f64;
        let x_padding = 0.3;
        let x_max = x_max_raw + x_padding;

        let y_min = 0.0;
        // let y_min = mse_values.iter().cloned().fold(f64::INFINITY, f64::min);
        let y_max_raw = mse_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let y_padding = (y_max_raw - y_min) * 0.05; // Add 5% padding
        let y_max = y_max_raw + y_padding;

        // Use the new function to create the plot and data structures
        let (plot, x_data, y_data) = plot::create_mse_plot(id, x_min, x_max, y_min, y_max);

        async move {
            // Animate the points
            for (i, mse) in mse_values.iter().enumerate() {
                let delay = i as u32 * 1000; // 1 second per animation step
                let window = window().unwrap();
                let id = id.to_string();
                let plot = Rc::clone(&plot);
                let x_data = Rc::clone(&x_data);
                let y_data = Rc::clone(&y_data);
                let x = indices[i];
                let y = *mse;

                let closure = Closure::wrap(Box::new(move || {
                    let id = id.clone();
                    let plot = Rc::clone(&plot); // Clone Rc for plot
                    let x_data = Rc::clone(&x_data);
                    let y_data = Rc::clone(&y_data);
                    spawn_local(async move {
                        // Update the x and y data
                        x_data.borrow_mut().push(x);
                        y_data.borrow_mut().push(y);

                        // Create a new scatter trace with the updated data
                        let scatter = Scatter::new(x_data.borrow().clone(), y_data.borrow().clone())
                            .mode(plotly::common::Mode::LinesMarkers) // Use Lines+Markers mode
                            .name("MSE Values");

                        // Re-render the plot
                        let mut plot = plot.borrow_mut();
                        let layout = plot.layout().clone();
                        *plot = Plot::new(); // Reset the plot
                        plot.add_trace(scatter); // Add the updated trace
                        plot.set_layout(layout); // Reapply the layout
                        plotly::bindings::react(&id, &plot).await;
                    });
                }) as Box<dyn Fn()>);

                window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        delay as i32,
                    )
                    .unwrap();

                closure.forget(); // Prevent memory leaks
            }

            Ok(())
        }
    });

    use_effect_with((), move |_| {
        p1.run();
        p2.run();
    });

    html! {
        <div style="display: flex; justify-content: space-around; width: 100%; height: 100%;">
            <div id="plot1" style="flex-grow: 1; min-width: 300px; height: 100%;"></div>
            <div id="plot2" style="flex-grow: 1; min-width: 300px; height: 100%;"></div>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
