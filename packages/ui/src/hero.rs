use api::save_dog;
use dioxus::prelude::*;

const HERO_CSS: Asset = asset!("/assets/styling/hero.css");
//const HEADER_SVG: Asset = asset!("/assets/header.svg");

#[component]
pub fn Hero() -> Element {

    let mut img_src = use_loader(|| async move {
        dioxus::Ok(
            reqwest::get("https://dog.ceo/api/breeds/image/random")
                .await?
                .json::<serde_json::Value>()
                .await?["message"]
                .to_string(),
        )
    })?;

    rsx! {
        document::Link { rel: "stylesheet", href: HERO_CSS }

        div { style: "text-align: center;",
            h1 { "Esta es la página de inicio de la página" }
        }     

        div { id : "dogview", 
            img { id : "dogimg", src : "{img_src}"}
        } 

        div { id : "buttons",

            button {  
                id : "skip", 
                onclick : move |_| img_src.restart(),
                "skip"
            }
            button { 
                id : "save",
                onclick : move |_| async move { _ = save_dog(img_src()).await}, 
                "save!!"
             }

        }

    }
}
