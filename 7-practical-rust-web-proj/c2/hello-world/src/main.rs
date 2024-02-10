use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world") // status code=200
}

#[actix_web::main] // executes main in a special runtime (actix-rt), built on top of Tokio RT
async fn main() -> std::io::Result<()>{
    println!("Listening on port 8000");
    HttpServer::new(|| {App::new().route("/hello", web::get().to(hello))}) // /hello -> HTTP GET to hello()
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
