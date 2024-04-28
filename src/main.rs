use actix_web::{web, App, HttpServer};
use controllers::register_controller::{confirm, finish, register};
use tera::Tera;

mod configs;
mod models;
mod services;
mod controllers;

#[actix_web::main]
async fn main()->std::io::Result<()>{
    let tera = Tera::new("src/views/**").expect("Teraのインスタン生成に失敗しました");
    HttpServer::new(move ||{
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(register)
            .service(confirm)
            .service(finish)   
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}