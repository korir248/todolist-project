use yew::prelude::*;


struct Book{
    id: u8,
    title: String,
    author: String,
    completed: bool,
}

// #[derive(Properties)]
#[function_component(App)]
 pub fn app() -> Html{
    let books: Vec<Book> = vec![Book{
        id: 1,
        author: "George R.R. Martin".to_string(),
        title: "In The House of the Worm".to_string(),
        completed: false
    },Book{
        id: 2,
        title: "The Dark Soul of the Night".to_string(),
        author: "Brian W. Aldiss".to_string(),
        completed: true

    },Book{
        id: 3,
        title: "Seeing".to_string(),
        author: "Harlan Ellison".to_string(),
        completed: true
    },Book{
        id: 4,
        title: "Predators".to_string(),
        author: "Steven Utley".to_string(),
        completed: false
    }
       ];

    let books_comp = books.iter().filter(|book| book.completed == true).map(|book| html! {
        <>
        
        <li>{ format!("Title: {}",book.title)}</li>
        <li>{ format!("Author: {}", book.author)}</li>
      <Button />
        </>
    }).collect::<Html>();


    let books_incomp = books.iter().filter(|book| book.completed == false).map(|book| html! {
        <>
        <ul> {format!("Id: {}  title:{} author: {}",book.id, book.title,book.author)}</ul> <Button/>
        </>

    }).collect::<Html>();

    
    html! {
        <>
        <div>
        <h3> { "Completed books"} </h3>
        <p> { books_comp }</p> 
        </div>
        <div>
        <h3> { "Incomplete books"} </h3>
        <p> { books_incomp }</p> 
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