#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

use std::env;
mod models;
mod schema;
use self::models::*;
use self::schema::cats::dsl::*;

// use serde_json::json; //enable json! (macro)
use actix_files::Files; 
use actix_web::{web, App, Error, HttpResponse, HttpServer};

use handlebars::{Handlebars, DirectorySourceOptions};

use diesel::prelude::*; 
use diesel::pg::PgConnection; 
use diesel::r2d2::{self, ConnectionManager};

// PgConnection comes from diesel::prelude
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

//use serde::{Serialize};
#[derive(Serialize)]
struct IndexTemplateData {
    project_name: String,
    cats: Vec<self::models::Cat>
}

async fn index(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool> ) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().expect("Can't get db connection from pool");
    
    
    let cats_data = web::block(move || {cats.limit(100).load::<Cat>(&mut connection)})
    .await
    .map_err(|_| { HttpResponse::InternalServerError().finish() });
    


    let data = IndexTemplateData {
        project_name: "Catdex".to_string(),
        cats: cats_data.unwrap().unwrap(),
    };
    let body = hb.render("index", &data).unwrap();

    Ok(HttpResponse::Ok().body(body))
    //Ok(HttpResponse::Ok().body("Teste"))


    /*
    let data = json!({
        "project_name": "Catdex", 
        "cats": [ 
            { 
                "name": "British short hair", 
                "image_path": "/static/image/british-short-hair.jpg"
            }, 
            { 
                "name": "Persian", 
                "image_path": "/static/image/persian.jpg" 
            }, 
            { 
                "name": "Ragdoll", 
                "image_path": "/static/image/ragdoll.jpg" 
            } 
        ] 
    });
    //anything that implements Serialize trait from serde can be rendered
    let body = hb.render("index", &data).unwrap(); 
    HttpResponse::Ok().body(body)
    */
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new(); //start handlebars template
    let options = DirectorySourceOptions {
        tpl_extension: ".html".to_string(),
        ..Default::default()
    };
    handlebars.register_templates_directory("./static", options).unwrap(); //register template directory. Needs dir_source feature enabled
    let handlebars_ref = web::Data::new(handlebars); // allow compiled templates to be shared accross threads

    //setup DB connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create DB connection pool.");

    println!("Listening on port 8080");
    HttpServer::new(move || {App::new().app_data(handlebars_ref.clone()).app_data(web::Data::new(pool.clone())).service(Files::new("/static", "static").show_files_listing(),)
        .route("/",web::get().to(index))})
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
