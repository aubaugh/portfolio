#![feature(proc_macro_hygiene)]

use wasm_bindgen::prelude::*;
use web_sys;

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
    video: Option<String>,
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
            let video = match &project.video {
                Some(path) => {
                    let mut webm: String = path.clone();
                    webm.push_str(".webm");
                    let mut mp4 = path.clone();
                    mp4.push_str(".mp4");
                    html! {
                        <video
                            muted="true"
                            loop="true"
                            controls="true"
                        >
                            <source src=webm type="video/webm" />
                            <source src=mp4 type="video/mp4" />
                            Your browser does not support HTML5 videos
                        </video>
                    }
                },
                None => VirtualNode::text(""),
            };
            html! {
                <div>
                    <hr />
                    <div class=PROJECT>
                        <div class=PROJECT_BANNER>
                            <a
                                href=project.url.to_string()
                                class=PROJECT_NAME
                            >{ project.name.to_string() }</a>
                            <span>
                                <b>Role: </b> { project.role.to_string() }
                            </span>
                        </div>
                        <p>{ project.description.to_string() }</p>
                        { video }
                        <div>
                            <b>Languages: </b> { languages }
                            <br />
                            <b>Technologies: </b> { technologies }
                        </div>
                    </div>
                </div>
            }
        }).collect();

        let mut email_url = String::from("mailto:");
        email_url.push_str(&config.email);

        let view = html! {
            <div class=CONTENT>
                <h1 class=CENTER>{ config.name }</h1>
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
        width: calc(100% - 100px);
        padding: 25px;
        border-radius: 25px;
        background-color: #ffffff8c;
        backdrop-filter: blur(10px);
        -webkit-backdrop-filter: blur(10px);
    }
"#};

static CENTER: &'static str = css!{r#"
    :host {
        text-align: center;
    }
"#};

static PROJECT: &'static str = css!{r#"
    :host {
        margin: 5px 0;
    }
"#};

static PROJECT_BANNER: &'static str = css!{r#"
    :host {
        display: flex;
        align-items: center;
        justify-content: space-between;
    }
"#};
static PROJECT_NAME: &'static str = css!{r#"
    :host {
        font-size: x-large;
        font-weight: bold;
    }
"#};

static _STYLE: &'static str = css!{r#"
    body {
        margin: 0;
        padding: 25px 0;
        width: 100%;
        height: 100%;
        font-family: "Arial", Courier, monospace;
    }

    p {
        text-indent: 2em;
    }

    video {
        width: 100%;
    }
"#};
