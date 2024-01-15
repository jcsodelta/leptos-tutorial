use leptos::*;

use crate::person::{Person, PersonOverview};

#[component]
pub fn App() -> impl IntoView {
    let age_leap = 3000;

    let (next_person_id, set_next_person_id) = create_signal(1);
    let create_person = move |name, age| -> Person {
        let person_id = next_person_id.get_untracked();
        set_next_person_id.update(|person_id| *person_id += 1);
        Person::new(person_id, name, age)
    };

    let (data, set_data) = create_signal(create_person(String::from("Jack"), 0));
    let derived_data_get = move || {
        let person = data.get();
        PersonOverview {
            description: format!("{} is {} year(s) old", person.name, person.age),
        }
    };
    let derived_data_get_untracked = move || {
        let person = data.get_untracked();
        PersonOverview {
            description: format!("{} is {} year(s) old", person.name, person.age),
        }
    };

    view! {
        <table border="1px">
            <tr>
                <th></th>
                <th>"tracked"</th>
                <th>"untracked"</th>
            </tr>
            <tr>
                <td>"Person (get)"</td>
                <td>{move || format!("{}", data.get())}</td>
                <td>{move || format!("{}", data.get_untracked())}</td>
            </tr>
            <tr>
                <td>"Person (with)"</td>
                <td>{move || data.with(|person| format!("{}", person))}</td>
                <td>{move || data.with_untracked(|person| format!("{}", person))}</td>
            </tr>
            <tr>
                <td>"Person Overview (get)"</td>
                <td>{move || format!("{}", derived_data_get())}</td>
                <td>{move || format!("{}", derived_data_get_untracked())}</td>
            </tr>
            <tr>
                <td>"Person Overview (with)"</td>
                <td>{move || format!("{}", derived_data_get())}</td>
                <td>{move || format!("{}", derived_data_get_untracked())}</td>
            </tr>
        </table>

        <div>
            <button on:click=move |_| {
                let mut person = data.get();
                person.age += 1;
                set_data.set(person);
            }>

                "Increase Age using set"
            </button>
        </div>
        <div>
            <button on:click=move |_| {
                set_data.update(|person| person.age += 1);
            }>

                "Increase Age using update"
            </button>
        </div>

        <div>
            <button on:click=move |_| {
                for _ in 0..age_leap {
                    let mut person = data.get();
                    person.age += 1;
                    set_data.set(person);
                }
            }>

                "Increase Age " {age_leap} " years using set"
            </button>
        </div>
        <div>
            <button on:click=move |_| {
                for _ in 0..age_leap {
                    set_data.update(|person| person.age += 1);
                }
            }>

                "Increase Age " {age_leap} " years using update"
            </button>
        </div>
        <div>
            <button on:click=move |_| {
                for _ in 0..age_leap {
                    let person = data.get();
                    let person = create_person(person.name, person.age + 1);
                    set_data.set(person);
                }
            }>

                "Increase Age " {age_leap} " years creating new structs"
            </button>
        </div>
    }
}
