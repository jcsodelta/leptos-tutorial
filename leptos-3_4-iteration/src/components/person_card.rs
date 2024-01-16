use std::time::Duration;
use leptos::*;

use crate::models::{person::Person, themes::Theme};

#[component]
pub fn PersonCard(person: ReadSignal<Person>, set_person: WriteSignal<Person>) -> impl IntoView {
    let (initializing, set_initializing) = create_signal(true);
    let initializing_message = move || {
        if initializing.get() {
            "initializing component..."
        } else {
            ""
        } 
    };

    let (danger_mode, set_danger_mode) = create_signal(false);
    let (font_size, set_font_size) = create_signal(FontSize::MEDIUM);
    let (theme, set_theme) = create_signal(Theme::Default);

    set_timeout(move || {
        set_initializing.set(false);
    }, Duration::from_secs(1));

    let change_theme_button = move |label, selected_theme: Theme| {
        view! {
            <button on:click=move |_| {
                set_theme.set(selected_theme.clone());
            }>{label}</button>
        }
    };

    view! {
        <div
            class="person-card"
            class:danger=move || danger_mode.get()
            data-theme=move || theme.with(|theme| theme.to_string())
        >
            <div>{move || { initializing_message() }}</div>

            <div class="person-card-buttons">
                <div>
                    <button on:click=move |_| {
                        set_person.update(|person| person.age += 1);
                    }>"Increase Age"</button>
                </div>
                <div>
                    <button on:click=move |_| {
                        set_danger_mode.set(!danger_mode.get());
                    }>"Toggle Danger"</button>
                </div>
            </div>

            <div>
                <div>Font:</div>
                <div class="flex">
                    <button on:click=move |_| {
                        set_font_size.set(FontSize::SMALL);
                    }>"small"</button>
                    <button on:click=move |_| {
                        set_font_size.set(FontSize::MEDIUM);
                    }>"medium"</button>
                    <button on:click=move |_| {
                        set_font_size.set(FontSize::LARGE);
                    }>"large"</button>
                </div>
            </div>

            <div>
                <div>Theme:</div>
                <div class="flex">
                    {change_theme_button("Light", Theme::Light)}
                    {change_theme_button("Default", Theme::Default)}
                    {change_theme_button("Dark", Theme::Dark)}
                </div>
            </div>

            <div>
                <hr/>
            </div>
            <div style:font-size=move || {
                font_size
                    .with(|value| match value {
                        FontSize::SMALL => ".7em",
                        FontSize::MEDIUM => "1em",
                        FontSize::LARGE => "1.3em",
                    })
            }>
                <div>"Name: " {person.with_untracked(|person| person.name.clone())}</div>
                <div>"Age: " {move || person.with(|person| person.age)}</div>
            </div>
        </div>
    }
}

enum FontSize {
    SMALL,
    MEDIUM,
    LARGE,
}
