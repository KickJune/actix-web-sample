use actix_web::web::Redirect;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
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

#[derive(Deserialize, Debug)]
struct ItemRequest {
    name: String,
    price: i32,
    description: Option<String>, // NULLが入るかもしれない時はOptionにする
}

#[derive(sqlx::FromRow, Deserialize, Debug)]
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

#[post("/new")]
async fn new(pool: web::Data<PgPool>, form: web::Form<ItemRequest>) -> HttpResponse {
    let item_request = form.into_inner();

    sqlx::query("INSERT INTO items(name, price, description) VALUES ($1, $2, $3)")
        .bind(item_request.name)
        .bind(item_request.price)
        .bind(item_request.description)
        .execute(pool.as_ref())
        .await
        .unwrap();

    HttpResponse::Found()
        .append_header(("Location", "/"))
        .finish()
}

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
            .service(new)
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
