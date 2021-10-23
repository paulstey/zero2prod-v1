//! src/routes/subscriptions.rs

use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use tracing;


#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}


pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    // Let's generate a random unique identifier 
    let request_id = Uuid::new_v4();

    // Spans, like logs, have an associated level 
    // `info_span` creates a span at the info-level
    // NOTE: the % symbol tells `tracing` crate to use Display 
    // implementation for logging purpose
    let request_span = tracing::info_span!(
        "Adding a new subscriber.",
        %request_id,        
        subscriber_email = %form.email,
        subscriber_name = %form.name 
    );

    let _request_span_guard = request_span.enter();

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - Saving new subscriber details in the database",
                request_id
            );
            HttpResponse::Ok().finish()
        },
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
