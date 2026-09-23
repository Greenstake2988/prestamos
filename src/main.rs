mod api;
mod excel;
mod fechas;
mod modelo;
use axum::{
    Router,
    routing::{get, post},
};

use crate::modelo::Prestamo;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
struct EstadoApp {
    prestamos: Arc<Mutex<Vec<Prestamo>>>,
}

#[tokio::main]
async fn main() {
    let estado = EstadoApp {
        prestamos: Arc::new(Mutex::new(Vec::new())),
    };

    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route(
            "/prestamos",
            post(api::crear_prestamo_handler).get(api::listar_prestamos_handler),
        )
        .route("/prestamos/{id}/tabla", get(api::tabla_prestamo_handler))
        .route("/prestamos/{id}/excel", get(api::excel_prestamo_handler))
        .with_state(estado);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Servidor corriendo en http://localhost:8000");
    axum::serve(listener, app).await.unwrap();
}
