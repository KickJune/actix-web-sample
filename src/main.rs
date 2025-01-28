use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use chrono::naive::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use tera::{Context, Tera};

#[get("/items/{id}")]
async fn item_detail(
    id: web::Path<i32>,
    tera: web::Data<Tera>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let row = sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id = $1")
        .bind(id.into_inner())
        .fetch_one(pool.as_ref())
        .await
        .unwrap();

    let mut context = Context::new();
    context.insert("item", &row);

    let rendered = tera.render("item-detail.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[derive(Debug, Deserialize)]
struct ItemRequest {
    name: String,
    price: i32,
    release_date: Option<NaiveDate>,
    description: Option<String>, // NULLが入るかもしれない時はOptionにする
}

#[derive(Debug, FromRow, Serialize)]
struct Item {
    id: i32,
    name: String,
    price: i32,
    release_date: Option<NaiveDate>,
    description: Option<String>, // NULLが入るかもしれない時はOptionにする
}

#[get("/items")]
async fn item_list(tera: web::Data<Tera>, pool: web::Data<PgPool>) -> HttpResponse {
    let rows = sqlx::query_as::<_, Item>("SELECT * FROM items")
        .fetch_all(pool.as_ref())
        .await
        .unwrap();

    let mut context = Context::new();
    context.insert("item_list", &rows);

    let rendered = tera.render("item-list.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[post("/new")]
async fn new_item(pool: web::Data<PgPool>, form: web::Form<ItemRequest>) -> HttpResponse {
    let item_request = form.into_inner();

    sqlx::query("INSERT INTO items(name, price, release_date, description) VALUES ($1, $2, $3, $4)")
        .bind(item_request.name)
        .bind(item_request.price)
        .bind(item_request.release_date)
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
        .expect("コネクションプール作成エラー");

    HttpServer::new(move || {
        let mut templates = Tera::new("templates/**/*").expect("Teraテンプレート設定エラー");
        templates.autoescape_on(vec!["tera"]);

        App::new()
            // "/"にアクセスされたら"/items"にリダイレクト
            .service(web::redirect("/", "/items"))
            .service(item_detail)
            .service(item_list)
            .service(new_item)
            .app_data(web::Data::new(templates))
            .app_data(web::Data::new(pool.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
