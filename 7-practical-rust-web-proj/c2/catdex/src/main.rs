#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;

use std::env;
use std::collections::HashMap;
mod models;
mod schema;
use self::models::*;
use self::schema::cats::dsl::*;

// use serde_json::json; //enable json! (macro)
use actix_files::Files; 
use actix_web::{http, web, App, Error, HttpResponse, HttpServer};

use handlebars::{Handlebars, DirectorySourceOptions};

use awmp::Parts;

use diesel::prelude::*; 
use diesel::pg::PgConnection; 
use diesel::r2d2::{self, ConnectionManager};
use diesel::result::Error as DieselError;

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

#[derive(Serialize)]
enum CatTemplateData {
    Success(models::Cat),
    Error(String), // You can customize the error type based on your needs
}

impl From<Result<models::Cat, DieselError>> for CatTemplateData {
    fn from(result: Result<models::Cat, DieselError>) -> Self {
        match result {
            Ok(cat) => CatTemplateData::Success(cat),
            Err(err) => CatTemplateData::Error(err.to_string()),
        }
    }
}

async fn add(hb: web::Data<Handlebars<'_>>) -> Result<HttpResponse, Error> {

    let body = hb.render("add", &{}).unwrap();

    Ok(HttpResponse::Ok().body(body))
}


async fn add_cat_form(pool: web::Data<DbPool>, mut parts: Parts) -> Result<HttpResponse, Error> {

    //warning: security issues with the way we save the files (can overwrite, can send malicious files, etc)
    let file_path = 
        parts.files
        .take("image")
        .pop()
        .and_then(|f| f.persist_in("./static/image").ok())
        .unwrap_or_default();
    
    let text_fields: HashMap<_, _> = parts.texts.as_pairs().into_iter().collect();
    
    //get a connection
    let mut connection = pool.get().expect("Can't get db connection from pool");
    let new_cat = NewCat {
        name: text_fields.get("name").unwrap().to_string(),
        image_path: file_path.to_string_lossy().to_string()
    };


    //insert a row into DB
    web::block(move || 
        diesel::insert_into(cats)
        .values(&new_cat)
        .execute(&mut connection)
    )
    .await
    .map_err(|_| {
        HttpResponse::InternalServerError().finish()
    }).unwrap();

    //return a proper response
    // 303 See itger -> redirect to new location
    Ok(HttpResponse::SeeOther().header(http::header::LOCATION, "/").finish())
}

async fn cat(hb: web::Data<Handlebars<'_>>, pool: web::Data<DbPool>, cat_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let mut connection = pool.get().expect("Can't get db connection from pool");

    let cat_data = web::block(move || cats.filter(id.eq(cat_id.into_inner())).first::<Cat>(&mut connection))
        .await
        .map_err(|_| {
            HttpResponse::InternalServerError().finish()
        });

    let template_data = CatTemplateData::from(cat_data.unwrap());

    let body = hb.render("cat", &template_data).unwrap();

    Ok(HttpResponse::Ok().body(body))
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
        .route("/",web::get().to(index))
        .route("/add",web::get().to(add))
        .route("/cat/{id}", web::get().to(cat))
        .route("/add_cat_form", web::post().to(add_cat_form))})
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
