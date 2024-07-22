use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer};

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    name: String,
}

impl FormData {
    fn new(email: String, name: String) -> Self {
        Self { email, name }
    }

    fn default() -> Self {
        Self {
            email: "".to_string(),
            name: "".to_string(),
        }
    }
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form_data: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
