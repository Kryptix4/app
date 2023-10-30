use leptos::{html::Input, *};

mod task;
use task::*;

mod list;
use list::List;

#[component]
fn App() -> impl IntoView {
    let (tasks, set_tasks) = create_signal(List::new());

    provide_context(set_tasks);

    let input_ref = create_node_ref::<Input>();
    let add_todo = move |event: web_sys::KeyboardEvent| {
        let input = input_ref.get().unwrap();
        let key_code = event.key_code();
        event.stop_propagation();

        const ENTER_KEY: u32 = 13;
        if key_code == ENTER_KEY {
            let text = input.value();
            let text = text.trim();

            if !text.is_empty() {
                set_tasks.update(|list| list.add(Task::new(text.to_string())));
            }

            input.set_value("");
        }
    };

    view! {
        <header>
            <h1>Active Tasks:</h1>
            <input
                placeholder="Input task"
                autofocus
                on:keydown=add_todo
                node_ref=input_ref
            />
            <button
                on:click=move |_| {
                    let text = input_ref.get().unwrap().value();
                    let text = text.trim();

                    if !text.is_empty() {
                        set_tasks.update(|list| list.add(Task::new(text.to_string())));
                    }

                    input_ref.get().unwrap().set_value("");
                }
            >
                "Add task"
            </button>
            <button
                on:click=move |_| set_tasks.update(|list| list.clear_done())
            >
                "Clear done"
            </button>
        </header>
        <section>
            <For
                each=move || tasks().0
                key=move |task| task.id
                children=move |task| {
                    let todo_input = create_node_ref::<Input>();

                    view!{
                        <div>
                        <input
                            node_ref=todo_input
                            class="toggle"
                            type="checkbox"
                            prop:checked=task.done
                            on:input={move |ev| {
                                let checked = event_target_checked(&ev);
                                task.done.set(checked);
                            }}
                        />
                        <p>{task.text}</p>
                        </div>
                    }
                }
            />
        </section>
        <footer>
        </footer>
    }
}

fn main() {
    mount_to_body(|| view! { <App/> })
}
