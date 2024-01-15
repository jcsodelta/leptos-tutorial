use leptos::*;

use app::App;

mod app;
mod person;

fn main() {
    mount_to_body(|| view! { <App/> })
}
