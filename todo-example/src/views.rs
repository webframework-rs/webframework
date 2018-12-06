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
        li {
            strong: &task.name;
            :" ";
            em: if task.done { "Done" } else { "Not Done" };
        }
    }
}

pub fn root(tasks: Vec<Task>) -> String {
    layout("Todos", html! {
        h1: "Todo App Example";
        h2: "Tasks:";
        ol(class="tasks") {
            @for task in tasks.iter() {
                : render_task(&task);
            }
        }
        form(method="POST", action="/create") {
            input(type="text", name="name");
            input(type="submit");
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

