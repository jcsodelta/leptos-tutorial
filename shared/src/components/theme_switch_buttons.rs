use leptos::*;

use crate::models::theme::Theme;

#[component]
pub fn ThemeSwitchButtons(
    #[prop(optional)] set_theme: Option<WriteSignal<Theme>>,
    #[prop(optional)] on_change_theme: Option<fn(Theme)>
) -> impl IntoView {
    let on_click = move |theme: Theme| {
        if let Some(set_theme) = set_theme {
            set_theme.set(theme.clone());
        }

        if let Some(on_change_theme) = on_change_theme {
            on_change_theme(theme);
        }
    };

    view! {
        <div class="flex">
            <button on:click=move |_| on_click(Theme::Dark)>"Dark"</button>
            <button on:click=move |_| on_click(Theme::Default)>"Default"</button>
            <button on:click=move |_| on_click(Theme::Light)>"Light"</button>
        </div>
    }
}
