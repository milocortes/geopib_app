use dioxus::prelude::*;


#[component]
pub fn Field(name : String) -> Element {
    rsx! {
        h1 { "HOLA!!!!"} 
        div { "Name: {name}" }
    }
}