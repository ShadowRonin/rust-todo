use eyre::Result;
use tokio::time::{sleep, Duration};
use serde::{Serialize, Deserialize};


use poem::{
    get,
    post,
    handler, 
    listener::TcpListener, 
    web::Path, 
    Route, 
    Server, 
    web::Json
};

#[tokio::main]
async fn main() -> Result<()> {
    // make run time errors colorful
    color_eyre::install()?;

    let app = Route::new()
    .at("/todo/:id", get(get_todo))
    .at("/todo/create", post(get_todo))
    .at("/todo", get(get_todos));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
      .run(app)
      .await?;

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: u64,
    title: String,
    description: String,
}

#[handler]
async fn create_todo(Json(todo): Json<Todo>) {
    sleep(Duration::from_millis(100)).await;

    println!("New todo: {:?}", todo);
}

#[handler]
async fn get_todos() -> Json<Vec<Todo>> {
    sleep(Duration::from_millis(100)).await;

    Json(vec!(
        Todo {
            id: 1,
            title: "Example".to_owned(),
            description: "this is an example".to_owned()
        },
        Todo {
            id: 2,
            title: "Example 2".to_owned(),
            description: "this is another example".to_owned()
        }
    ))
}

#[handler]
async fn get_todo(Path(name): Path<u64>) -> Json<Todo> {
    sleep(Duration::from_millis(100)).await;

    Json(Todo {
        id: 1,
        title: "Example".to_owned(),
        description: "this is an example".to_owned()
    })
}
