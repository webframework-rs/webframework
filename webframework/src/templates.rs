use horrorshow::helper::doctype;
use failure::Fail;

pub fn error_page(error: &Fail) -> String {
    (html! {
        : doctype::HTML;
        html {
            head {
                title: "An error occured";
            }
            body {
                @ if cfg!(debug_assertions) {
                    h1: error.to_string();
                    ol {
                        @ for fail in error.iter_causes() {
                            li: fail.to_string();
                        }
                    }
                } else {
                    h1: "We're sorry, but something went wrong";
                }

                small: "You can see this information because you're in a debug build.";
            }
        }
    }).to_string()
}

