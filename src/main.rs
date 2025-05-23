use actix_files as af;
use actix_web::{get, web, App, HttpServer, HttpResponse, Responder};
use actix_web::Result as ActixResult;
use actix_files::NamedFile;

use std::fs;

use rusqlite::{params, Connection, Result};

use std::sync::Mutex;

use std::io;

use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

use diesel::prelude::*;
use bcrypt::{hash, DEFAULT_COST};

mod models;
mod schema;
mod db;
mod handlers;

use db::init_pool;

use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //website available to all devices on network
    let ip1 = String::from("0.0.0.0:");
    //reads portnumber from .txt file
    let ip2 = fs::read_to_string("port_number.txt")?.trim().to_string();
    //concatenates port number to ip
    let ip3 = ip1 + &ip2;
    println!("Website running on port: {}", ip3);

    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    let pool = init_pool(&db_url);

    HttpServer::new(move || {
        App::new()
            
            .app_data(web::Data::new(pool.clone()))
            
            .service(
            af::Files::new("/static", "static")
            .show_files_listing()
            )

            .route("/about", web::get().to(serve_about))
            .route("/signup", web::get().to(serve_signup))
            .route("/contact", web::get().to(serve_contact))
            .route("/credits", web::get().to(serve_credits))
            .route("/home", web::get().to(serve_home))

            .route("/", web::get().to(serve_home))

            .route("/signup_account", web::post().to(handlers::signup))
            .route("/login_account", web::post().to(handlers::login_user))
            
            .default_service(
            af::Files::new("/", "static")
            .index_file("index.html")
            )
            
    })
    .bind(ip3)?
    .run()
    .await
}

async fn serve_home() -> ActixResult<NamedFile> {
    Ok(NamedFile::open("static/index.html")?)
}

async fn serve_about() -> ActixResult<NamedFile> {
    Ok(NamedFile::open("static/about.html")?)
}

async fn serve_signup() -> ActixResult<NamedFile> {
    Ok(NamedFile::open("static/signup.html")?)
}

async fn serve_contact() -> ActixResult<NamedFile> {
    Ok(NamedFile::open("static/contact.html")?)
}

async fn serve_credits() -> ActixResult<NamedFile> {
    Ok(NamedFile::open("static/credits.html")?)
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}