#[macro_use]
extern crate diesel;

mod models;
mod schema;
use self::models::*;
use self::schema::cats::dsl::*;
use self::schema::cats;

use actix_files::Files;
use actix_web::{web, App, error, HttpResponse, HttpServer, Responder, Error};
use serde::Serialize;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use std::env;


type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;


#[derive(Serialize)]
pub struct Cat {
 pub id: i32,
 pub name: String,
 pub image_path: String,
}

/*
async fn cats() -> impl Responder {
    let cats = vec![
        Cat {
            id: 1,
            name: "foo".to_string(),
            image_path: "foo.png".to_string(),
        },
        Cat {
            id: 2,
            name: "bar".to_string(),
            image_path: "bar.png".to_string(),
        },
    ];

    return web::Json(cats);
}
*/


async fn cats_endpoint(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let connection = pool.get().expect("Can't get db connection from pool");

    let cats_data = web::block(move || cats.limit(100).load::<Cat>(&connection))
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;
    
    return Ok(HttpResponse::Ok().json(cats_data));
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create DB connection pool.");


    println!("Listening on port 8080");
    HttpServer::new(move || {App::new()
            .service(web::scope("/api").route("/cats", web::get().to(cats_endpoint)),)
            .service(Files::new("/", "static").show_files_listing(),)
        })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}