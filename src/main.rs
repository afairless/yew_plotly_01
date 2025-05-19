use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;
use yew_hooks::use_async;
use std::rc::Rc;
use std::cell::RefCell;
mod utils;
mod plot;

#[function_component(App)]
pub fn plot_component() -> Html {

    let p1 = use_async::<_, _, ()>({
        let id = "plot1";
        let (plot, line_traces) = plot::create_plot();
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
        let (plot, line_traces) = plot::create_plot();
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
