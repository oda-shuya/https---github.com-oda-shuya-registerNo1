use actix_web::{get, post, web, HttpResponse, Responder };
use tera::{Context, Tera};
use crate::{models::entity::user_entity::FormData, services::register_service::{RegisterService, RegisterServiceTrait}};

async fn render_template(tera:web::Data<Tera>,template_name :&str,context:Context)->HttpResponse{
    tera.render(&template_name,&context)
        .map(|body| HttpResponse::Ok().content_type("text/html").body(body))
        .unwrap_or_else(|_| HttpResponse::InternalServerError().finish())
}

#[get("/")]
async fn register(tera : web::Data<Tera>)-> impl Responder{
    let context = Context::new();
    render_template(tera, "register.html",context).await
}

#[post("/confirm")]
async fn confirm(tera : web::Data<Tera>,form : web::Form<FormData>)-> impl Responder{
    let mut context = Context::new();
    context.insert("user",&form.into_inner());
    render_template(tera, "confirm.html", context).await
} 

#[post("/finish")]
async fn finish(tera : web::Data<Tera>,form : web::Form<FormData>)-> impl Responder{
    let mut context = Context::new();
    let service = RegisterService;
    context.insert("user",&form);
    let user = form.into_inner();
    match service.register(user){
        Ok(_) => render_template(tera, "finish.html", context).await,
        Err(e)=>{
            context.insert("error", &e);
            render_template(tera, "register.html", context).await
        }
    }
} 