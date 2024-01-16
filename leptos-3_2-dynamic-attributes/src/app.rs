use leptos::*;

use crate::person::Person;

enum FontSize {
    SMALL,
    MEDIUM,
    LARGE,
}

#[component]
pub fn App() -> impl IntoView {
    let (next_person_id, set_next_person_id) = create_signal(1);
    let create_person = |name, age| -> Person {
        let person_id = next_person_id.get_untracked();
        set_next_person_id.update(|person_id| *person_id += 1);
        Person::new(person_id, name, age)
    };

    let (light_theme, set_light_theme) = create_signal(true);
    let theme_name = move || {
        light_theme.with(|is_light_theme| match *is_light_theme {
            true => "light",
            false => "dark",
        })
    };

    let (danger_mode, set_danger_mode) = create_signal(false);
    let (font_size, set_font_size) = create_signal(FontSize::MEDIUM);

    let (data, set_data) = create_signal(create_person(String::from("Jack"), 0));

    view! {
        <div class="person-card" class:danger=move || danger_mode.get() data-theme=theme_name>
            <div class="person-card-buttons">
                <div>
                    <button on:click=move |_| {
                        set_data.update(|person| person.age += 1);
                    }>"Increase Age"</button>
                </div>
                <div>
                    <button on:click=move |_| {
                        set_light_theme.set(!light_theme.get());
                    }>"Toggle Theme"</button>
                </div>
                <div>
                    <button on:click=move |_| {
                        set_danger_mode.set(!danger_mode.get());
                    }>"Toggle Danger"</button>
                </div>
                <div>
                    <button on:click=move |_| {
                        set_font_size.set(FontSize::SMALL);
                    }>"Font: small"</button>
                </div>
                <div>
                    <button on:click=move |_| {
                        set_font_size.set(FontSize::MEDIUM);
                    }>"Font: medium"</button>
                </div>
                <div>
                    <button on:click=move |_| {
                        set_font_size.set(FontSize::LARGE);
                    }>"Font: large"</button>
                </div>
            </div>
            <div><hr /></div>
            <div style:font-size=move || font_size.with(|value| match value { FontSize::SMALL => ".7em", FontSize::MEDIUM => "1em", FontSize::LARGE => "1.3em" })>
                <div>"Name: " {data.with_untracked(|person| person.name.clone())}</div>
                <div>"Age: " {move || data.with(|person| person.age)}</div>
            </div>
        </div>
    }
}
