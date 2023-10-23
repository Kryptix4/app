use leptos::*;

mod task;
use task::*;

#[component]
fn App() -> impl IntoView {
    let list = move || {
        (1..=5)
            .map(|i| Task::new(format!("Row {}", i)))
            .collect::<Vec<_>>()
    };

    view! {
        <For
            each=list
            key=move |task| task.id
            children=move |task| {
                view!{
                    <p>{task.text}</p>
                }
            }
        />
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
