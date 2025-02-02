mod db;
mod handlers;

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::{PgPool};
use std::env;
use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // .envから設定値を読み込む
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("環境変数にDATABASE_URLがありません");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("コネクションプール作成エラー");

    let port_string = env::var("PORT").expect("環境変数にPORTがありません");
    let port = port_string.parse::<u16>().expect("環境変数にPORTの形式が不正です");

    HttpServer::new(move || {
        let mut templates = Tera::new("templates/**/*").expect("Teraテンプレート設定エラー");
        templates.autoescape_on(vec!["tera"]);

        App::new()
            // "/"にアクセスされたら"/items"にリダイレクト
            .service(web::redirect("/", "/items"))
            .service(handlers::item_detail)
            .service(handlers::item_list)
            .service(handlers::new_item)
            .app_data(web::Data::new(templates))
            .app_data(web::Data::new(pool.clone()))
            // 「/static/なんとか」にアクセスされたらstaticフォルダのファイルをレスポンスする
            .service(Files::new("/static", "./static"))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
