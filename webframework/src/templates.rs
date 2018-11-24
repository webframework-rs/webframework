use horrorshow::helper::doctype;
use failure::Fail;

#[cfg(debug_assertions)]
pub fn error_page(error: &Fail) -> String {
    (html! {
        : doctype::HTML;
        html {
            head {
                title: "An error occured";
            }
            body {
                h1: error.to_string();
                ol {
                    @ for fail in error.iter_chain() {
                        li: fail.to_string();
                    }
                }

                small: "You can see this information because you're in a debug build.";
            }
        }
    }).to_string()
}

#[cfg(not(debug_assertions))]
pub fn error_page(error: &Fail) -> String {
    (html! {
        : doctype::HTML;
        html {
            head {
                title: "An error occured";
            }
            body {
                h1: "We're sorry, but something went wrong";
            }
        }
    }).to_string()
}
