use actix_web::{App,web,get,post,HttpResponse,HttpServer,Responder};
use serde::{Serialize,Deserialize};
use tera::{Tera, Context};

#[derive(Serialize,Deserialize,Clone)]
struct FormData{
    username : String,
    mailaddress : String,
    password : String
}

#[get("/")]
async fn register(tera : web::Data<Tera>)-> impl Responder{
    let context = Context::new();
    tera.render("register.html",&context)
        .map(|body| HttpResponse::Ok().content_type("text/html").body(body))
        .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
}

#[post("/confirm")]
async fn confirm(tera : web::Data<Tera>,form : web::Form<FormData>)-> impl Responder{
    let mut context = Context::new();
    context.insert("user",&form.into_inner());
    tera.render("confirm.html",&context)
        .map(|body| HttpResponse::Ok().content_type("text/html").body(body))
        .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
} 

#[post("/finish")]
async fn finish(tera : web::Data<Tera>,form : web::Form<FormData>)-> impl Responder{
    let mut context = Context::new();
    context.insert("user",&form.into_inner());
    tera.render("finish.html",&context)
        .map(|body| HttpResponse::Ok().content_type("text/html").body(body))
        .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
} 

#[actix_web::main]
async fn main()->std::io::Result<()>{
    let tera = Tera::new("src/**").expect("Teraのインスタン生成に失敗しました");
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