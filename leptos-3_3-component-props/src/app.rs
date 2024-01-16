use leptos::logging::error;
use leptos::*;

use crate::components::person_card::PersonCard;
use crate::models::person::Person;
use crate::models::themes::Theme;

#[component]
pub fn App() -> impl IntoView {
    let (next_person_id, set_next_person_id) = create_signal(1);
    let create_person = |name, age| -> Person {
        let person_id = next_person_id.get_untracked();
        set_next_person_id.update(|person_id| *person_id += 1);
        Person::new(person_id, name, age)
    };

    let (theme, set_theme) = create_signal(Theme::Default);

    let (person_1, set_person_1) = create_signal(create_person(String::from("Jessie"), 0));
    let (person_2, set_person_2) = create_signal(create_person(String::from("James"), 0));

    let change_theme_button = move |label, selected_theme: Theme| {
        view! {
            <button on:click=move |_| {
                set_theme.set(selected_theme.clone());
                let results = leptos::document().get_elements_by_tag_name("html");
                for result_i in 0..results.length() {
                    match results.get_with_index(result_i) {
                        None => {
                            error!("Error trying to fing html tag instance");
                        }
                        Some(html_tag) => {
                            match html_tag
                                .set_attribute("data-theme", &theme.with(|value| value.to_string()))
                            {
                                Err(err) => {
                                    error!("{:?}", err);
                                }
                                Ok(_) => {}
                            }
                        }
                    }
                }
            }>{label}</button>
        }
    };

    view! {
        <div class="flex">
            {change_theme_button("Light", Theme::Light)}
            {change_theme_button("Default", Theme::Default)}
            {change_theme_button("Dark", Theme::Dark)}
        </div>
        <div class="person-card-group">
            <PersonCard person=person_1 set_person=set_person_1/>
            <PersonCard person=person_1 set_person=set_person_1/>
            <PersonCard person=person_2 set_person=set_person_2/>
        </div>
    }
}
