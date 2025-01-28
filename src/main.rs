use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use askama::Template;
use askama_actix::TemplateToResponse;
use chrono::naive::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};

#[derive(Template)]
#[template(path = "hello.askama.html")]
struct HelloTemplate {
    name: String,
}
#[get("/hello/{name}")]
async fn hello(name: web::Path<String>) -> HttpResponse {
    let hello = HelloTemplate {
        name: name.into_inner(),
    };
    hello.to_response()
}

#[derive(Template)]
#[template(path = "item-list.askama.html")]
struct ItemListTemplate {
    item_list: Vec<Item>,
}

// #[derive(Debug, Deserialize, Serialize)]
// struct Task {
//     id: Option<String>,
//     name: Option<String>,
//     price: Option<i32>,
// }

#[derive(sqlx::FromRow)]
struct Item {
    id: i32,
    name: String,
    price: i32,
    description: Option<String>, // NULLが入るかもしれない時はOptionにする
}

#[get("/")]
async fn item_list(pool: web::Data<PgPool>) -> HttpResponse {
    let rows = sqlx::query_as::<_, Item>("SELECT * FROM items")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();

    let item_list_template = ItemListTemplate { item_list: rows };
    item_list_template.to_response()
}

// #[post("/update")]
// async fn update(pool: web::Data<PgPool>, form: web::Form<Task>) -> HttpResponse {
//     let task = form.into_inner();
//
//     if let Some(id) = task.id {
//         sqlx::query("DELETE FROM tasks WHERE task = $1")
//             .bind(id)
//             .execute(pool.as_ref())
//             .await
//             .unwrap();
//     }
//     match task.name {
//         Some(task) if !task.is_empty() => {
//             sqlx::query("INSERT INTO tasks (task) VALUES ($1)")
//                 .bind(task)
//                 .execute(pool.as_ref())
//                 .await
//                 .unwrap();
//         }
//         _ => {}
//     }
//
//     HttpResponse::Found()
//         .append_header(("Location", "/"))
//         .finish()
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres://sample_user:sample_pass@localhost:5432/sample_db")
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .service(hello)
            // .service(update)
            .service(item_list)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
