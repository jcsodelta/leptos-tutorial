use leptos::*;

use app::App;

mod app;
mod models {
    pub mod person;
    pub mod themes;
}
mod components {
    pub mod person_card;
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
