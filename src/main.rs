use leptos::{html::Input, *};

mod components;
use components::*;

#[component]
fn App() -> impl IntoView {
    let (tasks, set_tasks) = create_signal(List::new());

    let input_node = create_node_ref::<Input>();

    let get_input = move || {
        input_node.get().unwrap().value()
    };

    view! {
        <h1>Active Tasks:</h1>

        <input
            placeholder="Input task"
            autofocus
            node_ref=input_node
        />

        <button on:click=move |_| set_tasks.update(|list| {
            let text = get_input();

            if !text.is_empty() {
                list.add(text.trim());
            }
        })>Add task</button>

        <button on:click=move |_| set_tasks.update(|list| {
            let text = get_input();

            if !text.is_empty() {
                list.remove(text.trim());
            }
        })>Remove task</button>

        <br/>

        <button on:click=move |_| set_tasks.update(|list| {
            list.0.clear();
        })>Clear all</button>

        <button on:click=move |_| set_tasks.update(|list| {
            list.clear_completed();
        })>Clear completed</button>

        <For each=move || tasks().0 key=move |task| task.id children=move |task| {
            let todo_input = create_node_ref::<Input>();
            let text = task.text.clone();

            view! {
                <div>
                <input
                    node_ref=todo_input
                    class="toggle"
                    type="checkbox"
                    prop:checked=task.done
                    on:input={move |ev| {
                        task.done.set(event_target_checked(&ev));
                    }}
                />

                <p>{text}</p>

                <button on:click=move |_| set_tasks.update(|list| {
                    list.remove(task.text.clone());
                })>Remove</button>
                </div>
            }
        }/>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
