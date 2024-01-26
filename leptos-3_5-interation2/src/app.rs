use std::cmp::{max, min};

use leptos::logging::{error, log};
use leptos::*;

use shared::components::person_card::PersonCard;
use shared::components::theme_switch_buttons::ThemeSwitchButtons;
use shared::models::person::Person;
use shared::person_data::PersonData;

const LIST_DISPLAY_LENGTH: usize = 5;

#[component]
pub fn App(person_data: ReadSignal<PersonData>) -> impl IntoView {
    let (dynamic_vector, set_dynamic_vector) = create_signal::<Vec<Person>>(vec![]);
    let (current_index, set_current_index) = create_signal(0);

    let change_current_index = move |index_diff: i8| {
        person_data.with(|person_data| {
            let persons = person_data.get_persons();
            let max_position = persons.len();
            let lower_position: usize = max(
                0,
                min(
                    current_index.get_untracked() + index_diff,
                    (max_position - 1) as i8,
                ),
            ) as usize;
            let upper_position: usize =
                max(0, min(lower_position + LIST_DISPLAY_LENGTH, max_position)) as usize;
            match persons.get(lower_position..upper_position) {
                None => {
                    error!("Error updating current index");
                }
                Some(persons_slice) => {
                    set_current_index.set(lower_position as i8);
                    let persons_slice = persons_slice.to_vec();
                    set_dynamic_vector.set(persons_slice);
                }
            }
        });
    };

    change_current_index(0);

    view! {
        <ThemeSwitchButtons on_change_theme=|theme| {
            let results = leptos::document().get_elements_by_tag_name("html");
            for result_i in 0..results.length() {
                match results.get_with_index(result_i) {
                    None => {
                        error!("Error trying to fing html tag instance");
                    }
                    Some(html_tag) => {
                        match html_tag.set_attribute("data-theme", &theme.to_string()) {
                            Err(err) => {
                                error!("{:?}", err);
                            }
                            Ok(_) => {}
                        }
                    }
                }
            }
        }/>

        <p>"Only 5 items (using dynamic rendering)"</p>
        <div class="person-card-list-container">
            <button on:click=move |_| {
                change_current_index(-1);
            }>"<"</button>
            <div class="person-card-list">
                <For
                    each=move || dynamic_vector.get()
                    key=|person| person.get_id()
                    children=move |person| {
                        view! { <PersonCard person=&person /> }
                    }
                />

            </div>
            <button on:click=move |_| {
                change_current_index(1);
            }>">"</button>
        </div>
    }
}
