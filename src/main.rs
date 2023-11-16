use std::error::Error;

use eyre::Result;
use tokio::time::{sleep, Duration};
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() -> Result<()> {
    // make run time errors colorful
    color_eyre::install()?;

    println!("Hello, world!");

    let new_todo = get_todo().await?; 
    println!("{:?}", new_todo);

    panic!("test panic!"); // test color eyre

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: u64,
    title: String,
    description: String,
}

async fn get_todo() -> Result<Todo> {
    sleep(Duration::from_millis(100)).await;

    Ok(Todo {
        id: 1,
        title: "Example".to_owned(),
        description: "this is an example".to_owned()
    })
}
