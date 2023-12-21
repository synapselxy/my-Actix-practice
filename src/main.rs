use actix_web::{get, post, web, App, HttpReponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpReponse::Ok().body("Hello World!")
}

#[post("/echo")]
async fn echo(req_boday: String) -> impl Responder {
    HttpReponse::Ok().body(req_boday)
}

async fn manual_hello() -> impl Responder {
    HttpReponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey",web::get().to(manual_hello))
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}