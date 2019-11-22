#![feature(proc_macro_hygiene)]

use wasm_bindgen::prelude::*;
use web_sys;
#[cfg(target_arch = "wasm32")]
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
                .iter().enumerate().map(|(index, language)| {
                let separator = if index + 1 < project.languages.len() {
                    VirtualNode::text(" \u{2662} ")
                } else {
                    VirtualNode::text("")
                };
                html! {
                    <span><em><a
                        href=config.languages[&language.to_string()].to_string()
                    >{ language.to_string() }</a></em>{
                        separator
                    }</span>
                }
            }).collect();
            let technologies: Vec<VirtualNode> = project.technologies
                .iter().enumerate().map(|(index, technology)| {
                let separator = if index + 1 < project.technologies.len() {
                    VirtualNode::text(" \u{2662} ")
                } else {
                    VirtualNode::text("")
                };
                html! {
                    <span><em><a
                        href=config.technologies[&technology.to_string()].to_string()
                    >{ technology.to_string() }</a></em>{
                        separator
                    }</span>
                }
            }).collect();
            html! {
                <div class=PROJECT>
                    <div>
                        <a
                            href=project.url.to_string()
                            class=PROJECT_NAME
                        >{ project.name.to_string() }</a>
                        <span class=RIGHT>
                            <b>Role: </b> { project.role.to_string() }
                        </span>
                    </div>
                    <p>{ project.description.to_string() }</p>
                    <div>
                        <b>Languages: </b> { languages }
                        <br />
                        <b>Technologies: </b> { technologies }
                    </div>
                </div>
            }
        }).collect();

        let mut email_url = String::from("mailto:");
        email_url.push_str(&config.email);

        let view = html! {
            <div class=CONTENT>
                <h2 class=CENTER>{ config.name }</h2>
                <div class=CENTER><em><a href=email_url>{ config.email }</a></em></div>
                <p>{ config.about }</p>
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

static CONTENT: &'static str = css!{r#"
    :host {
        margin: 0 auto;
        max-width: 700px;
        padding: 25px;
        background-color: #999999;
        color: #000000;
    }
"#};

static CENTER: &'static str = css!{r#"
    :host {
        text-align: center;
    }
"#};

static RIGHT: &'static str = css!{r#"
    :host {
        float: right;
    }
"#};

static PROJECT: &'static str = css!{r#"
    :host {
        padding: 25px;
        margin: 5px auto;
        background-color: #cccccc;
    }
"#};

static PROJECT_NAME: &'static str = css!{r#"
    :host {
        font-size: large;
        font-weight: bold;
    }
"#};

static _STYLE: &'static str = css!{r#"
    body {
        margin: 0;
        padding: 0;
        width: 100%;
        height: 100%;
        background-color: #333333;
        font-family: Georgia, serif;
    }

    p {
        text-indent: 2em;
    }
"#};
