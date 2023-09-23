mod todolist;
use todolist::service;

use actix_web::{get, web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::io::Result;



struct AppState {
    todolist_entries: Mutex<Vec<TodolistEntry>>
}

#[derive(Serialize, Deserialize, Clone)]
struct TodolistEntry {
    id: i32,
    date: i64,
    title: String
}

#[get("/")]
async fn index() -> String {
    "This is a health check".to_string()
}

#[actix_web::main]
async fn main() -> Result<()> {
    let app_data = web::Data::new(AppState {
            todolist_entries: Mutex::new(vec![])
        }
    );

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(service::config)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}



fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
                  <title>GCD Calculator</title>
                  <form action="/gcd" method="post">
                  <input type="text" name="n"/>
                  <input type="text" name="m"/>
                  <button type="submit">Compute GCD</button>
                  </form>
            "#,
        )
}
