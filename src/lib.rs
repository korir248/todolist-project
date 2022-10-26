use yew::prelude::*;


struct Todo{
    id: u8,
    name: String,
    completed: bool,
}


#[function_component(App)]
 pub fn app() -> Html{
    let todos: Vec<Todo> = vec![Todo{
           id: 1,
           name: String::from("Wash dishes"),
           completed: true
       },Todo{
        id: 2,
        name: String::from("Cook supper"),
        completed: false
    },Todo{
        id: 3,
        name: String::from("learn rust"),
        completed: false
    },Todo{
        id: 4,
        name: String::from("Watch a movie"),
        completed: false
    }
       ];
    let todos_comp = todos.iter().filter(|todo| todo.completed == true).map(|todo| html! {
        <>
        <ul> {format!("Id: {}  name: {}",todo.id, todo.name)}</ul> <Button/>
        </>
    }).collect::<Html>();
    let todos_incomp = todos.iter().filter(|todo| todo.completed == false).map(|todo| html! {
        <>
        <ul> {format!("Id: {}  name:{}",todo.id, todo.name)}</ul> <Button/>
        </>

    }).collect::<Html>();
    html! {
        <>
        <div>
        <h3> { "Completed Todos"} </h3>
        <p> { todos_comp }</p> 
        </div>
        <div>
        <h3> { "Incomplete Todos"} </h3>
        <p> { todos_incomp }</p> 
        </div>
        </>
    }
}

#[function_component(Button)]
pub fn button() -> Html {
    html! {
        <button> { "delete"}</button>
    }
}