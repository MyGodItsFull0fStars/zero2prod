use actix_web::{web, HttpResponse};

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

pub async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}
