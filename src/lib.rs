// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use serde::Deserialize;
use std::collections::HashMap;
use seed::{prelude::*, *};

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

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    ron::de::from_str(include_str!("../static/config.ron")).unwrap()
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
type Model = Portfolio;

// ------ ------
//    Update
// ------ ------

// (Remove the line below once any of your `Msg` variants doesn't implement `Copy`.)
#[derive(Copy, Clone)]
// `Msg` describes the different events you can modify state with.
enum Msg {}

// `update` describes how to handle each `Msg`.
fn update(_msg: Msg, _model: &mut Model, _: &mut impl Orders<Msg>) {
    ()
}

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    let mut projects: Vec<Node<Msg>> = model
        .projects
        .iter()
        .map(|project| {
            let languages: Vec<Node<Msg>> = project.languages.iter().enumerate()
                .map(|(index, language)| {
                    let separator = if index > 0 {
                        String::from(" \u{2662} ")
                    } else {
                        String::new()
                    };

                    span![
                        em![
                            a![
                                attrs! {
                                    At::Href => &model.languages[language]
                                },
                                &language
                            ],
                            span![&separator]
                        ]
                    ]
                }).collect();

                let technologies: Vec<Node<Msg>> = project.technologies
            .iter().enumerate().map(|(index, technology)| {
            let separator = if index > 0 {
                String::from(" \u{2662} ")
            } else {
                String::new()
            };

            span![
                em![
                    a![
                        attrs! {
                            At::Href => &model.technologies[technology]
                        },
                        &technology
                    ],
                    span![&separator]
                ]
            ]
        }).collect();

        let video = match &project.video {
            Some(path) => video![
                style! {
                    St::Width => "100%"
                },
                attrs! {
                    At::Src => &path,
                    At::Loop => true,
                    At::Controls => true,
                },
                "Your browser does not support HTML5 videos"
            ],
            None => empty![]
        };

        div![
            hr![],
            div![
                style! {
                    St::Margin => "5px 0"
                },
                div![
                    style! {
                        St::Display => "flex",
                        St::AlignItems => "center",
                        St::JustifyContent => "space-between",
                    },
                    a![
                        attrs! {
                            At::Href => &project.url
                        },
                        style! {
                            St::FontSize => "x-large",
                            St::FontWeight => "bold",
                        },
                        &project.name
                    ],
                    span![
                        b!["Role: "],
                        span![&project.role]
                    ]
                ],
                p![
                    style! {
                        St::TextIndent => "2em"
                    },
                    &project.description
                ],
                video,
                div![
                    b!["Languages: "],
                    span![languages],
                    br![],
                    b!["Technologies: "],
                    span![technologies]
                ]
            ]
        ]
    }).collect();

    let mut email_url = String::from("mailto:");
    email_url.push_str(&model.email);

    projects.insert(
        0,
        h1![
            style! {
                St::TextAlign => "center"
            },
            &model.name
        ]
    );
    projects.insert(
        1,
        div![
            style! {
                St::TextAlign => "center"
            },
            em![
                a![
                    attrs! {
                        At::Href => &email_url
                    },
                    &model.email
                ]
            ]
        ]
    );
    projects.insert(
        2,
        p![
            style! {
                St::TextIndent => "2em"
            },
            &model.about
        ]
    );

    div![
        style! {
            St::Padding => "25px 0",
            St::Width => "100%",
            St::Height => "100%",
            St::BackgroundImage => "url(static/assets/background.jpg)",
            St::BackgroundRepeat => "no-repeat",
            St::BackgroundSize => "cover",
            St::BackgroundAttachment => "fixed",
        },
        div![
            style! {
                St::Margin => "0 auto",
                St::MaxWidth => "700px",
                St::Width => "calc(100% - 100px)",
                St::Padding => "25px",
                St::BorderRadius => "25px",
                St::BackgroundColor => "#ffffff8c",
                St::BackdropFilter => "blur(10px)",
                //TODO: Not available atm
                //St::WebkitBackdropFilter => "blur(10px)",
            },
            projects
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
