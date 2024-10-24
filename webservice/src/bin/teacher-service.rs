use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::sync::Mutex;
use std::{env, io};

#[path = "../dbaccess/mod.rs"]
mod dbaccess;

#[path = "../state.rs"]
mod state;

#[path = "../routers.rs"]
mod routers;

#[path = "../models/mod.rs"]
mod models;

#[path = "../handlers/mod.rs"]
mod handlers;

#[path = "../errors.rs"]
mod errors;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    let shared_data = web::Data::new(AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        // courses: Mutex::new(vec![]),
        db: pool,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_route)
            .configure(course_routes)
    };

    HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}
