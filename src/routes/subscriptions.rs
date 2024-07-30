use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
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
    // Spans like logs have an associated level
    // `info_span` creates a span at the info level
    let request_span = tracing::info_span!(
        "Adding new subscriber.",
        // this notation is called `structured information`
        // the % before a value tells `tracing` to use the `Display` implementation of the value
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    );

    // Using `enter` in an async function is a recipe for disaster!
    // So don't try this at home!!
    let _request_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving new subscriber details in the database");

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
    // First we attach the instrumentation, then we `await` it
    .instrument(query_span)
    .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
