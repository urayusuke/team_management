use actix_web::{get, App, HttpResponse, HttpServer, web};
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};


#[get("/")]
async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32]) // <- クッキー鍵
                    .name("auth-cookie")
                    .secure(false),
            ))
            .service(hello)
            .configure(crate::routes::auth::init_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
