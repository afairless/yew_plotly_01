use yew::prelude::*;
mod utils;
mod plot;

#[function_component(App)]
pub fn plot_component() -> Html {
    let p = yew_hooks::use_async::<_, _, ()>({
        let id = "plot-div";
        let plot = plot::create_plot();

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
