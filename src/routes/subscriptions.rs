use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

impl FormData {
    pub fn new(email: String, name: String) -> Self {
        Self { email, name }
    }

    pub fn default() -> Self {
        Self {
            email: "".to_string(),
            name: "".to_string(),
        }
    }
}

pub async fn subscribe(
    form: web::Form<FormData>,
    // Retrieving a connection from the application state!
    pool: web::Data<PgPool>,
) -> HttpResponse {
    // generate a random unique identifier
    let request_id = Uuid::new_v4();
    tracing::info!(
        "request_id {} - Adding '{}' '{}' as a new subscriber",
        request_id,
        form.email,
        form.name,
    );
    tracing::info!(
        "request_id {} - Saving new subscriber details in the database",
        request_id
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now(),
    )
    // Using the pool as a drop-in replacement for a single DBConnection
    // Allows us to pool the queries concurrently
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
