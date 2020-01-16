use dominator::{class, html, Dom};
use futures_signals::signal::always;
use lazy_static::lazy_static;
use ron::de::from_str;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

/// The configuration structure that is included and deserialized from `config.ron`
#[derive(Debug, Deserialize)]
struct Portfolio {
    name: String,
    email: String,
    about: String,
    languages: HashMap<String, String>,
    technologies: HashMap<String, String>,
    projects: Vec<Project>,
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

struct State {
    config: Portfolio,
}

impl State {
    fn new() -> Arc<Self> {
        let config_file = include_str!("../static/config.ron");
        Arc::new(Self {
            config: from_str(config_file).unwrap(),
        })
    }

    fn render(state: Arc<Self>) -> Dom {
        lazy_static! {
            static ref BACKGROUND: String = class! {
                .style("padding", "25px 0")
                .style("width", "100%")
                .style("height", "100%")
                .style(
                    "background-image",
                    "url(assets/background.jpg)"
                )
                .style("background-repeat", "no-repeat")
                .style("background-size", "cover")
                .style("background-attachment", "fixed")
            };
            static ref CONTENT: String = class! {
                .style("margin", "0 auto")
                .style("max-width", "700px")
                .style("width", "calc(100% - 100px)")
                .style("padding", "25px")
                .style("border-radius", "25px")
                .style("background-color", "#ffffff8c")
                .style_signal([
                    "backdrop-filter",
                    "-webkit-backdrop-filter"
                ], always("blur(10px)"))
            };
            static ref CENTER: String = class! {
                .style("text-align", "center")
            };
            static ref PROJECT: String = class! {
                .style("margin", "5px 0")
            };
            static ref PROJECT_BANNER: String = class! {
                .style("display", "flex")
                .style("align-items", "center")
                .style("justify-content", "space-between")
            };
            static ref PROJECT_NAME: String = class! {
                .style("font-size", "x-large")
                .style("font-weight", "bold")
            };
            static ref PARAGRAPH: String = class! {
                .style("text-indent", "2em")
            };
            static ref VIDEO: String = class! {
                .style("width", "100%")
            };
        }

        let mut projects: Vec<Dom> =
            state
                .config
                .projects
                .iter()
                .map(|project| {
                    let mut languages: Vec<Dom> = project.languages
                .iter().enumerate().map(|(index, language)| {
                let separator = if index + 1 < project.languages.len() {
                    String::from(" \u{2662} ")
                } else {
                    String::new()
                };

                html!("span", {
                    .children(&mut [
                        html!("em", {
                            .children(&mut [
                                html!("a", {
                                    .attribute("href", &state.config.languages[language])
                                    .text(&language)
                                }),
                                html!("span", {
                                    .text(&separator)
                                })
                            ])
                        })
                    ])
                })
            }).collect();

                    let mut technologies: Vec<Dom> = project.technologies
                .iter().enumerate().map(|(index, technology)| {
                let separator = if index + 1 < project.technologies.len() {
                    String::from(" \u{2662} ")
                } else {
                    String::new()
                };

                html!("span", {
                    .children(&mut [
                        html!("em", {
                            .children(&mut [
                                html!("a", {
                                    .attribute("href", &state.config.technologies[technology])
                                    .text(&technology)
                                }),
                                html!("span", {
                                    .text(&separator)
                                })
                            ])
                        })
                    ])
                })
            }).collect();

                    let video = match &project.video {
                        Some(path) => html!("video", {
                            .class(&*VIDEO)
                            .attribute("src", &path)
                            .attribute("loop", "true")
                            .attribute("controls", "true")
                            .text("Your browser does not support HTML5 videos")
                        }),
                        None => Dom::empty(),
                    };

                    html!("div", {
                        .children(&mut [
                            html!("hr"),
                            html!("div", {
                                .class(&*PROJECT)
                                .children(&mut [
                                    html!("div", {
                                        .class(&*PROJECT_BANNER)
                                        .children(&mut [
                                            html!("a", {
                                                .attribute("href", &project.url)
                                                .class(&*PROJECT_NAME)
                                            }),
                                            html!("span", {
                                                .children(&mut [
                                                    html!("b", {
                                                        .text("Role: ")
                                                    }),
                                                    html!("span", {
                                                        .text(&project.role)
                                                    })
                                                ])
                                            })
                                        ])
                                    }),
                                    html!("p", {
                                        .class(&*PARAGRAPH)
                                        .text(&project.description)
                                    }),
                                    video,
                                    html!("div", {
                                        .children(&mut [
                                            html!("b", {
                                                .text("Languages: ")
                                            }),
                                            html!("span", {
                                                .children(&mut languages)
                                            }),
                                            html!("br"),
                                            html!("b", {
                                                .text("Technologies: ")
                                            }),
                                            html!("span", {
                                                .children(&mut technologies)
                                            })
                                        ])
                                    })
                                ])
                            })
                        ])
                    })
                })
                .collect();

        let mut email_url = String::from("mailto:");
        email_url.push_str(&state.config.email);

        projects.insert(
            0,
            html!("h1", {
                .class(&*CENTER)
                .text(&state.config.name)
            }),
        );
        projects.insert(
            1,
            html!("div", {
                .class(&*CENTER)
                .children(&mut [
                    html!("em", {
                        .children(&mut [
                            html!("a", {
                                .attribute("href", &email_url)
                                .text(&state.config.email)
                            })
                        ])
                    })
                ])
            }),
        );
        projects.insert(
            2,
            html!("p", {
                .class(&*PARAGRAPH)
                .text(&state.config.about)
            }),
        );

        html!("div", {
            .class(&*BACKGROUND)
            .children(&mut [
                html!("div", {
                    .class(&*CONTENT)
                    .children(&mut projects)
                })
            ])
        })
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console_log::init_with_level(log::Level::Debug).unwrap();

    let state = State::new();

    dominator::append_dom(&dominator::body(), State::render(state));

    Ok(())
}
