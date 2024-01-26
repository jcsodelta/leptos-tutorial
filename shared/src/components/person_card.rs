use std::time::Duration;
use leptos::*;

use crate::{models::{person::Person, theme::Theme, font_size::FontSize}, components::theme_switch_buttons::ThemeSwitchButtons};

#[component]
pub fn PersonCard<'a>(person: &'a mut Person) -> impl IntoView {
    let (initializing, set_initializing) = create_signal(true);
    let initializing_message = move || {
        if initializing.get() {
            "initializing component..."
        } else {
            ""
        } 
    };
    set_timeout(move || {
        set_initializing.set(false);
    }, Duration::from_secs(1));

    let (danger_mode, set_danger_mode) = create_signal(false);
    let (font_size, set_font_size) = create_signal(FontSize::MEDIUM);
    let (theme, set_theme) = create_signal(Theme::Default);

    let (person_signal, set_person_signal) = create_signal(person);

    // let increase_age = || {
    //     person.age += 1;
    // };

    view! {
        <div
            class="person-card"
            class:danger=move || danger_mode.get()
            data-theme=move || theme.with(|theme| theme.to_string())
        >
            <div>
                <div>theme</div>
                <ThemeSwitchButtons set_theme />
            </div>


            <div>
                <div>font</div>
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
                <div>age</div>
                <div>
                    <div>
                        <button on:click=move |_| {
                            set_person_signal.update(|person| {
                                person.age += 1;
                            })
                        }>"Increase"</button>
                    </div>
                </div>
            </div>

            <div>
                <div>danger</div>
                <div>
                    <div>
                        <button on:click=move |_| {
                            set_danger_mode.set(!danger_mode.get());
                        }>"Toggle Danger"</button>
                    </div>
                </div>
            </div>

            <div>
                <hr />
            </div>

            <div style:font-size=move || {
                font_size
                    .with(|value| match value {
                        FontSize::SMALL => ".7em",
                        FontSize::MEDIUM => "1em",
                        FontSize::LARGE => "1.3em",
                    })
            }>
                <div>"Name: " {person_signal.with(|person| person.name.clone())}</div>
                <div>"Age: " {person_signal.with(|person| person.age)}</div>
            </div>

            <div>{move || { initializing_message() }}</div>
        </div>
    }
}
