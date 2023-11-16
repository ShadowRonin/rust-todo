use chrono::{DateTime, Local};
use sqlx::{MySqlPool};

use serde::{Serialize, Deserialize};

// TODO: really should be using eyre::Result for everything...
// use eyre::Result;

use poem::{
    listener::TcpListener, 
    Route,
    web::Data,
    EndpointExt,
    error::InternalServerError,
    Result,
};

use poem_openapi::{
    OpenApi, 
    OpenApiService,
    payload::Json,
    Object,
    param::Path,
};

#[derive(Clone, Debug, Deserialize, Object, Serialize)]
struct TodoNew {
    title: String,
    description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Object, Serialize)]
struct Todo {
    id: u64,
    title: String,
    description: Option<String>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
}

struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/todo", method = "post")]
    async fn create_todo(&self, pool: Data<&MySqlPool>, Json(todo): Json<TodoNew>) -> Result<Json<u64>> {
        let id = sqlx::query_as!(
	        Todo, 
	        "INSERT INTO todo (title, description) values (?, ?)",
            todo.title,
            todo.description,
	    )
		.execute(pool.0)
		.await
		.map_err(InternalServerError)?
		.last_insert_id();

        Ok(Json(id))
    }

    #[oai(path = "/todo/:id", method = "get")]
    async fn get_todo(&self, pool: Data<&MySqlPool>, Path(id): Path<u64>) -> Result<Json<Todo>> {
        let todo = sqlx::query_as!(
	        Todo, 
	        "SELECT * FROM todo WHERE id = ?",
            id
	    )
		.fetch_one(pool.0)
		.await
        // TODO: error should be logged by eyre
		.map_err(InternalServerError)?;
    
        Ok(Json(todo))
    }

    #[oai(path = "/todo", method = "get")]
    async fn get_todos(&self, pool: Data<&MySqlPool>) -> Result<Json<Vec<Todo>>> {
        let todos = sqlx::query_as!(
	        Todo, 
	        "SELECT * FROM todo"
	    )
		.fetch_all(pool.0)
		.await
        // TODO: error should be logged by eyre
		.map_err(InternalServerError)?;
    
        Ok(Json(todos))
    }
}


#[tokio::main]
async fn main() -> eyre::Result<()> {
    // make run time errors colorful
    color_eyre::install()?;

    let pool = 
	    MySqlPool::connect("mysql://myuser:mypassword@localhost/mydatabase").await?;
    let api_service = OpenApiService::new(Api, "Manage Todos", "1.0").server("http://127.0.0.1:3000/api");
    let ui = api_service.openapi_explorer();
    let app = Route::new()
        .nest("/api", api_service)
        .nest("/", ui)
        .data(pool);

    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await?;

    Ok(())
}
