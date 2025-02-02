use chrono::NaiveDate;
use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow, Serialize)]
pub struct Item {
    id: i32,
    name: String,
    price: i32,
    release_date: Option<NaiveDate>,
    description: Option<String>, // NULLが入るかもしれない時はOptionにする
}

pub async fn select_all_items(pool: &PgPool) -> Vec<Item> {
    sqlx::query_as::<_, Item>("SELECT * FROM items")
        .fetch_all(pool)
        .await
        .unwrap()
}

pub async fn select_item_by_id(pool: &PgPool, id: i32) -> Item {
    sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id = $1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}

#[derive(Debug)]
pub struct NewItem {
    pub name: String,
    pub price: i32,
    pub release_date: Option<NaiveDate>,
    pub description: Option<String>,
}

pub async fn insert_item(pool: &PgPool, new_item: NewItem) -> Item {
    sqlx::query_as::<_, Item>(
        "INSERT INTO items(name, price, release_date, description)
              VALUES ($1, $2, $3, $4)
              RETURNING *",
    )
    .bind(&new_item.name)
    .bind(&new_item.price)
    .bind(&new_item.release_date)
    .bind(&new_item.description)
    .fetch_one(pool)
    .await
    .unwrap()
}
