use askama::Template;
use chrono::{DateTime, Local};
use mime::Mime;
use sqlx::MySqlPool;

use serde::{Serialize, Deserialize};

use std::{env, path::PathBuf};

use poem::{
    listener::TcpListener, 
    Route,
    web::{Data, Accept},
    EndpointExt,
    error::InternalServerError,
    Result,
    endpoint::StaticFileEndpoint, 
    IntoResponse,
    Response,
};

use poem_openapi::{
    OpenApi, 
    OpenApiService,
    payload::{Json, Html},
    Object,
    param::Path, ApiResponse,
};

pub mod templates;

#[derive(Clone, Debug, Deserialize, Object, Serialize)]
struct TodoNew {
    title: String,
    description: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Object, Serialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
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
		.map_err(InternalServerError)?;
    
        Ok(Json(todo))
    }

    #[oai(path = "/todo", method = "get")]
    async fn get_todos(&self, pool: Data<&MySqlPool>, accept: Accept) -> Result<TodosResult> {
        let todos = sqlx::query_as!(
	        Todo, 
	        "SELECT * FROM todo"
	    )
		.fetch_all(pool.0)
		.await
		.map_err(InternalServerError)?;

        if accept.0.iter().any(|x| x.subtype().as_str() == "html") {
            let todo = todos.first().unwrap();

            let template = templates::TodoRow {
                title: todo.title.as_str(),
                desc: todo.description.as_ref().map(|x| x.as_str()).unwrap_or(""),
            };

            Ok(TodosResult::Html(Html(template.render().unwrap())))
        }
        else {
            Ok(TodosResult::Json(Json(todos)))
        }
    }
}

#[derive(ApiResponse, Debug, Clone)]
pub enum TodosResult {
    #[oai(status = 200)]
    Json(Json<Vec<Todo>>),
    #[oai(status = 200)]
    Html(Html<String>),
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // make run time errors colorful
    color_eyre::install()?;

    let pool = 
	    MySqlPool::connect("mysql://myuser:mypassword@localhost/mydatabase").await?;
    let api_service = OpenApiService::new(Api, "Manage Todos", "1.0").server("http://localhost:3000/api");
    let docs = api_service.openapi_explorer();
    let app = Route::new()
        .nest("/api", api_service)
        .nest("/docs", docs)
        .nest("/", 
        StaticFileEndpoint::new("./assets/index.html")
        )
        .data(pool);

    poem::Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await?;

    Ok(())
}



