#![feature(proc_macro_hygiene)]

use wasm_bindgen::prelude::*;
use web_sys;
use web_sys::MouseEvent;

use css_rs_macro::css;
use virtual_dom_rs::prelude::*;

use ron::de::from_str;
use serde::Deserialize;

/// The configuration structure that is included and deserialized from `config.ron`
#[derive(Debug, Deserialize)]
struct ProjectConfig {
    name: String,
    description: String,
    url: String,
}
#[derive(Debug, Deserialize)]
struct PortfolioConfig {
    name: String,
    about: String,
    projects: Vec<ProjectConfig>
}

#[wasm_bindgen]
struct App {
  dom_updater: DomUpdater
}

#[wasm_bindgen]
impl App {
    #[wasm_bindgen(constructor)]
    pub fn new () -> App {
        let config_file = include_str!("../config.ron");
        let config: PortfolioConfig = from_str(config_file).unwrap();
        let projects: Vec<VirtualNode> = config.projects.iter().map(|project| {
            html! {
                <div>
                    <a href=project.url.to_string()>{ project.name.to_string() }</a>
                    <p>{ project.description.to_string() }</p>
                </div>
            }
        }).collect();

        let view = html! {
            <div>
                <h2>{ config.name }</h2>
                <h3>About: { config.about }</h3>
                { projects }
            </div>
        };

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();

        let dom_updater = DomUpdater::new_append_to_mount(view, &body);

        App { dom_updater }
    }
}

static _STYLE: &'static str = css!{r#"
    h2, h3 {
        text-align: center;
    }

    div {
        border-radius: 25px;
        border: 2px solid #ccc;
        padding: 25px;
        margin: 5px auto;
        max-width: 75%;
    }
"#};
