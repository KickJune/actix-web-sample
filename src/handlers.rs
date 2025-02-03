use crate::db;
use actix_web::web::{Data, Form, Path};
use actix_web::{get, post, HttpResponse};
use chrono::NaiveDate;
use serde::Deserialize;
use sqlx::PgPool;
use tera::{Context, Tera};

#[get("/items")]
pub async fn item_list(tera: Data<Tera>, pool: Data<PgPool>) -> HttpResponse {
    let item_list = db::select_all_items(&pool).await;

    let mut context = Context::new();
    context.insert("item_list", &item_list);

    let rendered = tera.render("item-list.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[get("/items/{id}")]
pub async fn item_detail(id: Path<i32>, tera: Data<Tera>, pool: Data<PgPool>) -> HttpResponse {
    let item = db::select_item_by_id(&pool, id.into_inner()).await;

    let mut context = Context::new();
    context.insert("item", &item);

    let rendered = tera.render("item-detail.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[derive(Debug, Deserialize)]
struct ItemForm {
    name: String,
    price: i32,
    release_date: String,
    description: String,
}

#[get("/new")]
pub async fn new_item(tera: Data<Tera>) -> HttpResponse {
    let context = Context::new();
    let rendered = tera.render("item-new.tera", &context).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[post("/new")]
pub async fn create_new_item(pool: Data<PgPool>, form: Form<ItemForm>) -> HttpResponse {

    let item_request = form.into_inner();

    db::insert_item(
        &pool,
        db::NewItem {
            name: item_request.name,
            price: item_request.price,
            release_date: if item_request.release_date.is_empty() {
                None
            } else {
                Some(NaiveDate::parse_from_str(&item_request.release_date, "%Y-%m-%d").unwrap())
            },
            description: if item_request.description.is_empty() {
                None
            } else {
                Some(item_request.description)
            },
        },
    )
    .await;

    HttpResponse::Found()
        .append_header(("Location", "/items"))
        .finish()
}
