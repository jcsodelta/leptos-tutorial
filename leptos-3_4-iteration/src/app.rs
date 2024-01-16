use std::cmp::{max, min};

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

    let (data_vector, _) = create_signal(vec![
        create_signal(create_person(String::from("Alberto"), 0)),
        create_signal(create_person(String::from("Bento"), 0)),
        create_signal(create_person(String::from("Carlos"), 0)),
        create_signal(create_person(String::from("Diego"), 0)),
        create_signal(create_person(String::from("Evandro"), 0)),
        create_signal(create_person(String::from("Fábio"), 0)),
        create_signal(create_person(String::from("Gustavo"), 0)),
        create_signal(create_person(String::from("Humberto"), 0)),
        create_signal(create_person(String::from("Iago"), 0)),
        create_signal(create_person(String::from("João"), 0)),
        create_signal(create_person(String::from("Kleber"), 0)),
        create_signal(create_person(String::from("Luiz"), 0)),
        create_signal(create_person(String::from("Marcelo"), 0)),
        create_signal(create_person(String::from("Neto"), 0)),
        create_signal(create_person(String::from("Osvaldo"), 0)),
        create_signal(create_person(String::from("Pedro"), 0)),
        create_signal(create_person(String::from("Quitério"), 0)),
        create_signal(create_person(String::from("Rogério"), 0)),
        create_signal(create_person(String::from("Simão"), 0)),
        create_signal(create_person(String::from("Tiago"), 0)),
        create_signal(create_person(String::from("Umberto"), 0)),
        create_signal(create_person(String::from("Vagner"), 0)),
        create_signal(create_person(String::from("Wagner"), 0)),
        create_signal(create_person(String::from("Xerxes"), 0)),
        create_signal(create_person(String::from("Yago"), 0)),
        create_signal(create_person(String::from("Zeca"), 0)),
    ]);

    let (static_vector, set_static_vector) = create_signal(data_vector.get_untracked());
    let (dynamic_vector, set_dynamic_vector) = create_signal(data_vector.get_untracked());

    let list_display_length = 5;
    let (current_index, set_current_index) = create_signal(0);
    let change_current_index = move |index_diff: i8| {
        let max_position = data_vector.with_untracked(|data_vector| data_vector.len()) - 1;
        let lower_position: usize = max(
            0,
            min(
                current_index.get_untracked() + index_diff,
                max_position as i8,
            ),
        ) as usize;
        let upper_position: usize =
            max(0, min(lower_position + list_display_length, max_position)) as usize;
        data_vector.with_untracked(|data_vector| {
            match data_vector.get(lower_position..upper_position) {
                None => {
                    error!("Error updating current index");
                }
                Some(data) => {
                    set_current_index.set(lower_position as i8);
                    set_static_vector.set(data.to_vec());
                    set_dynamic_vector.set(data.to_vec());
                }
            }
        });
    };

    change_current_index(0);

    view! {
        <div class="flex">
            {change_theme_button("Light", Theme::Light)}
            {change_theme_button("Default", Theme::Default)}
            {change_theme_button("Dark", Theme::Dark)}
        </div>
        <p>All items:</p>
        <div class="person-card-list">
            {data_vector
                .get_untracked()
                .into_iter()
                .map(|(person, set_person)| {
                    view! { <PersonCard person set_person/> }
                })
                .collect_view()}
        </div>

        <p>"Only 5 items (using static view)"</p>
        <div class="person-card-list-container">
            <button on:click=move |_| {
                change_current_index(-1);
            }>"<"</button>
            <div class="person-card-list">
                {move || {
                    static_vector
                        .get()
                        .into_iter()
                        .map(|(person, set_person)| {
                            view! { <PersonCard person set_person/> }
                        })
                        .collect_view()
                }}
            </div>
            <button on:click=move |_| {
                change_current_index(1);
            }>">"</button>
        </div>

        <p>"Only 5 items (using dynamic rendering)"</p>
        <div class="person-card-list-container">
            <button on:click=move |_| {
                change_current_index(-1);
            }>"<"</button>
            <div class="person-card-list">
                <For
                    each=move || { dynamic_vector.get() }
                    key=|(person, _set_person)| person.with_untracked(|person| person.name.clone())
                    children=move |(person, set_person)| {
                        view! { <PersonCard person set_person/> }
                    }
                />

            </div>
            <button on:click=move |_| {
                change_current_index(1);
            }>">"</button>
        </div>
    }
}
