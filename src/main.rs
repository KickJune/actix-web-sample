use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use askama::Template;
use askama_actix::TemplateToResponse;
use serde::Deserialize;
use sqlx::{FromRow, PgPool};

#[derive(Template)]
#[template(path = "item-detail.askama.html")]
struct ItemDetailTemplate {
    item: Item,
}
#[get("/items/{id}")]
async fn hello(id: web::Path<i32>, pool: web::Data<PgPool>) -> HttpResponse {
    let row = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id = $1")
        .bind(id.into_inner())
        .fetch_one(pool.as_ref())
        .await
        .unwrap();

    let template = ItemDetailTemplate { item: row };
    template.to_response()
}

#[derive(Debug, Template)]
#[template(path = "item-list.askama.html")]
struct ItemListTemplate {
    item_list: Vec<Item>,
}

#[derive(Debug, Deserialize)]
struct ItemRequest {
    name: String,
    price: i32,
    description: Option<String>, // NULLが入るかもしれない時はOptionにする
}

#[derive(Debug, FromRow)]
struct Item {
    id: i32,
    name: String,
    price: i32,
    description: Option<String>, // NULLが入るかもしれない時はOptionにする
}

#[get("/items")]
async fn item_list(pool: web::Data<PgPool>) -> HttpResponse {
    let rows = sqlx::query_as::<_, Item>("SELECT * FROM items")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();

    let template = ItemListTemplate { item_list: rows };
    template.to_response()
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
        .append_header(("Location", "/items"))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPool::connect("postgres://sample_user:sample_pass@localhost:5432/sample_db")
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .service(web::redirect("/", "/items"))
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
