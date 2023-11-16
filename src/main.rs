use eyre::Result;
use tokio::time::{sleep, Duration};
use serde::{Serialize, Deserialize};

use poem::{
    listener::TcpListener, 
    Route, 
};

use poem_openapi::{
    OpenApi, 
    OpenApiService,
    payload::Json,
    Object,
    param::Path
};

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/todo", method = "post")]
    async fn create_todo(&self, Json(todo): Json<Todo>) {
        sleep(Duration::from_millis(100)).await;
    
        println!("New todo: {:?}", todo);
    }

    #[oai(path = "/todo/:id", method = "get")]
    async fn get_todo(&self, Path(id): Path<u64>) -> Json<Todo> {
        sleep(Duration::from_millis(100)).await;
    
        Json(Todo {
            id: 1,
            title: "Example".to_owned(),
            description: "this is an example".to_owned()
        })
    }

    #[oai(path = "/todo", method = "get")]
    async fn get_todos(&self) -> Json<Vec<Todo>> {
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
}


#[tokio::main]
async fn main() -> Result<()> {
    // make run time errors colorful
    color_eyre::install()?;

    let api_service = OpenApiService::new(Api, "Manage Todos", "1.0").server("http://127.0.0.1:3000/api");
    let ui = api_service.openapi_explorer();
    let app = Route::new()
        .nest("/api", api_service)
        .nest("/", ui);

    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await?;

    Ok(())
}

#[derive(Clone, Debug, Deserialize, Object, Serialize)]
struct Todo {
    id: u64,
    title: String,
    description: String,
}
