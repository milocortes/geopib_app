use charming::{
    component::{Axis, Grid, Legend, Title},
    element::{AxisPointer, AxisPointerType, AxisType, Tooltip, Trigger},
    series::Bar,
    series::Line,
    Chart, WasmRenderer,
};
use dioxus::prelude::*;

const HERO_CSS: Asset = asset!("/assets/styling/hero.css");
//const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {
    let renderer_ts: Signal<WasmRenderer> = use_signal(|| WasmRenderer::new(1500, 700));

    let mut data_ts = use_loader(api::list_precip)?;
    let total_data: Vec<String> = (1..data_ts.len()).map(|x| x.to_string()).collect();

    let mut serie_ts = use_loader(api::list_serie)?;

    let mut time_index: Vec<String> = vec![];

    for (time, value) in serie_ts.cloned() {
        time_index.push(time.format("%Y-%m-%d").to_string());
    }

    use_effect(move || {
        let chart_ts = Chart::new()
            .title(Title::new().text("CO2 Histórico para El Salvador"))
            .tooltip(Tooltip::new().trigger(Trigger::Axis))
            .x_axis(
                Axis::new()
                    .type_(AxisType::Category)
                    .data(time_index.clone()),
            )
            .y_axis(Axis::new().type_(AxisType::Value))
            .series(Line::new().smooth(true).data(data_ts.cloned()));
        renderer_ts
            .read_unchecked()
            .render("chart_ts", &chart_ts)
            .unwrap();
    });

    rsx! {
        document::Link { rel: "stylesheet", href: HERO_CSS }

        div { style: "text-align: center;",
            h1 { "Esta es la página de inicio de la página" }
        }

        div { style: "width: 100%; text-align: center;",
            div { id: "chart_ts", style: "display: inline-block;" }
        }
        /*
        ul {
            for item in data.cloned(){
                li { "{item}" }
            }
        }
        */
    }
}
