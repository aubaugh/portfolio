#![feature(proc_macro_hygiene)]

use wasm_bindgen::prelude::*;
use web_sys;
use web_sys::MouseEvent;

use css_rs_macro::css;
use virtual_dom_rs::prelude::*;
use std::collections::HashMap;

use ron::de::from_str;
use serde::Deserialize;

/// The configuration structure that is included and deserialized from `config.ron`
#[derive(Debug, Deserialize)]
struct Portfolio {
    name: String,
    email: String,
    about: String,
    languages: HashMap<String, String>,
    technologies: HashMap<String, String>,
    projects: Vec<Project>
}
#[derive(Debug, Deserialize)]
struct Project {
    name: String,
    role: String,
    languages: Vec<String>,
    technologies: Vec<String>,
    description: String,
    url: String,
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
        let config: Portfolio = from_str(config_file).unwrap();
        let projects: Vec<VirtualNode> = config.projects
            .iter().map(|project| {
            let languages: Vec<VirtualNode> = project.languages
                .iter().map(|language| {
                html! {
                    <a href=config.languages[&language.to_string()].to_string()>
                        { language.to_string() }
                    </a>
                }
            }).collect();
            let technologies: Vec<VirtualNode> = project.technologies
                .iter().map(|technology| {
                html! {
                    <a href=config.technologies[&technology.to_string()].to_string()>
                        { technology.to_string() }
                    </a>
                }
            }).collect();
            html! {
                <div>
                    <p>
                        <a href=project.url.to_string() class="inline">
                            { project.name.to_string() }
                        </a>
                        <span class="right">
                            <b>Role:</b> { project.role.to_string() }
                        </span>
                    </p>
                    <p>{ project.description.to_string() }</p>
                    <p>
                        Languages: <em> { languages } </em>
                        <span class="right">
                            Technologies: <em> { technologies } </em>
                        </span>
                    </p>
                </div>
            }
        }).collect();

        let mut email_url = String::from("mailto:");
        email_url.push_str(&config.email);

        let view = html! {
            <div>
                <h2>{ config.name }</h2>
                <em><a href=email_url>{ config.email }</a></em>
                <p><b>About: </b>{ config.about }</p>
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
    h2 {
        text-align: center;
    }

    div {
        border-radius: 25px;
        border: 2px solid #ccc;
        padding: 25px;
        margin: 5px auto;
        max-width: 75%;
    }

    a {
        margin-right: 5px;
    }

    .right {
        float: right;
    }
"#};
