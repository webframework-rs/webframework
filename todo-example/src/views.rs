use horrorshow::prelude::*;
use horrorshow::helper::doctype;

use crate::models::Task;

fn layout<I: Render>(title: &str, inner: I) -> String {
    (html! {
        : doctype::HTML;
        html {
            head {
                title: title;
            }
            body {
                : &inner;
            }
        }
    }).to_string()
}

fn render_task<'a>(task: &'a Task) -> Box<Render + 'a> {
    box_html! {
        strong: &task.name;
    }
}

pub fn root(tasks: Vec<Task>) -> String {
    layout("Todos", html! {
        h1: "Todo App Example";
        h2: "Tasks:";
        ol(class="tasks") {
            @for task in tasks.iter() {
                li {
                    : render_task(&task);
                }
            }
        }
    })
}

pub fn not_found(path: &str) -> String {
    layout("Not Found", html! {
        h1: "Oops, we could not find that!";
        p {
            : "We're sorry, but we couldn't find ";
            : path;
            : ". You can go back ";
            a(href="/"): "Home.";
        }
    })
}

