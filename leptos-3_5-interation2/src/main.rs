use leptos::*;

use shared::person_data::PersonData;

use app::App;

mod app;

fn main() {
    mount_to_body(|| {
        let (person_data, _) = create_signal(PersonData::new());

        view! { <App person_data/> }
    })
}
