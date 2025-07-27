use std::{env, net::SocketAddr};

use axum::Router;
use sea_orm::{ Database, DatabaseConnection};

mod routes;
mod database;

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection
}

#[tokio::main]
async fn main() {

    unsafe {
        env::set_var("RUST_LOG", "debug");
    }

    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    
    let conn: DatabaseConnection = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let state = AppState{ conn };

    let app = Router::new()
    .merge(routes::words_route())
    .merge(routes::login_route())
    .with_state(state);

    // Get the port number from the environment, default to 3000
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string()) // Get the port as a string or default to "3000"
        .parse() // Parse the port string into a u16
        .expect("Failed to parse PORT");

    // Create a socket address (IPv6 binding)
    let address = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

    println!("Application running on {}", listener.local_addr().unwrap());

    // Run the app with hyper, listening on the specified address
    axum::serve(listener, app).await.unwrap();
}
