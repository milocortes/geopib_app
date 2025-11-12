use dioxus::prelude::*;

use ui::{Navbar, NavbarNav, NavbarTrigger, NavbarContent, NavbarItem};
use views::{Blog, Home, Field};

mod views;


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(WebNavbar)]
    #[route("/")]
    Home {  },
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/field/:name")]
    Field {
        name : String
    }
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Router::<Route> {}
    }
}

/// A web-specific Router around the shared `Navbar` component
/// which allows us to use the web-specific `Route` enum.
#[component]
fn WebNavbar() -> Element {
    rsx! {

        /* 
        Navbar {
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }
        */
        div { class: "navbar-example",
            Navbar { aria_label: "Components",
                NavbarItem {
                    index: 2usize,
                    value: "home".to_string(),
                    to: Route::Home { },
                    "Home"
                }

                NavbarNav { index: 0usize,
                    NavbarTrigger { "Geoespaciales" }
                    NavbarContent { class: "navbar-content",
                        NavbarItem {
                            index: 0usize,
                            value: "calendar".to_string(),
                            to: Route::Field { name: "CO2".to_string() } ,
                            "Emisiones de CO2 (Gg)"
                        }
                        NavbarItem {
                            index: 1usize,
                            value: "slider".to_string(),
                            to: Route::Home {  },
                            "Temperatura del aire (°C)"
                        }
                        NavbarItem {
                            index: 2usize,
                            value: "checkbox".to_string(),
                            to: Route::Home {  },
                            "Temperatura de la superficie terrestre (°C)"
                        }
                        NavbarItem {
                            index: 3usize,
                            value: "radio_group".to_string(),
                            to: Route::Home {  },
                            "Precipitacion (milímetros)"
                        }
                        NavbarItem {
                            index: 3usize,
                            value: "radio_group".to_string(),
                            to: Route::Home {  },
                            "NDBI (Normalized Difference Built-up Index)"
                        }
                        NavbarItem {
                            index: 3usize,
                            value: "radio_group".to_string(),
                            to: Route::Home {  },
                            "NDVI (Normalized Difference Vegetation Index) - MODIS (GEE)"
                        }
                        NavbarItem {
                            index: 3usize,
                            value: "radio_group".to_string(),
                            to: Route::Home {  },
                            "EVI (Enhanced Vegetation Index)"
                        }
                        NavbarItem {
                            index: 3usize,
                            value: "radio_group".to_string(),
                            to: Route::Home {  },
                            "NDVI (Normalized Difference Vegetation Index) - WFP"
                        }
                        NavbarItem {
                            index: 3usize,
                            value: "radio_group".to_string(),
                            to: Route::Home {  },
                            "Desastres naturales"
                        }
                        NavbarItem {
                            index: 3usize,
                            value: "radio_group".to_string(),
                            to: Route::Home {  },
                            "Luces nocturnas VIIRS Black Marble"
                        }
                        NavbarItem {
                            index: 3usize,
                            value: "radio_group".to_string(),
                            to: Route::Home {  },
                            "Luces nocturnas VIIRS Colorado School of Mines"
                        }
                        NavbarItem {
                            index: 3usize,
                            value: "radio_group".to_string(),
                            to: Route::Home {  },
                            "Luces nocturnas DMSP Colorado School of Mines"
                        }                                                                                                                                                
                    }
                }
                NavbarNav { index: 1usize,
                    NavbarTrigger { "Administrativas" }
                    NavbarContent {
                        NavbarItem {
                            index: 0usize,
                            value: "toast".to_string(),
                            to: Route::Home {  },
                            "PIB trimestral en USD"
                        }
                        NavbarItem {
                            index: 1usize,
                            value: "tabs".to_string(),
                            to: Route::Home {  },
                            "PIB, Índices de volumen encadenados."
                        }
                        NavbarItem {
                            index: 2usize,
                            value: "dialog".to_string(),
                            to: Route::Home {  },
                            "Exportaciones FOB "
                        }
                        NavbarItem {
                            index: 3usize,
                            value: "alert_dialog".to_string(),
                            to: Route::Home {  },
                            "Importaciones CIF"
                        }
                        NavbarItem {
                            index: 4usize,
                            value: "tooltip".to_string(),
                            to: Route::Home {  },
                            "Remesas"
                        }
                    }
                }

            }
        }

        Outlet::<Route> {}
    }
}
