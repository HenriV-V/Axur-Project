use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use serde_json::json;

use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Telefone {
    numero: String,
    tipo_denuncia: Option<String>,
}

pub async fn create_telefone(
    pool: web::Data<PgPool>,
    form: web::Json<Telefone>,
) -> HttpResponse {
    match sqlx::query!(
    r#"
    insert into telefones (id, numero, tipo_denuncia, data_denuncia)
    values ($1, $2, $3, $4)
    "#,
    Uuid::new_v4(),
    form.numero,
    form.tipo_denuncia,
    Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            let response = json!({
                "message": "Denuncia registrada com sucesso!"
            });
            HttpResponse::Created()
                .content_type("application/json")
                .json(response)
        },
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn get_telefone(
    pool: web::Data<PgPool>,
    numero: web::Path<String>, // Receives the phone number from the URL
) -> HttpResponse {
    match sqlx::query_as!(
        Telefone,
        r#"
        SELECT numero, tipo_denuncia
        FROM telefones
        WHERE numero = $1
        "#,
        numero.into_inner()
    )
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(telefone)) => HttpResponse::Ok().json(telefone),
        Ok(None) => HttpResponse::NotFound().body("Telefone nao encontrado"),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn list_telefones(
    pool: web::Data<PgPool>,
) -> HttpResponse {
    match sqlx::query_as!(
        Telefone,
        r#"
        SELECT numero, tipo_denuncia
        FROM telefones
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(telefones) => HttpResponse::Ok().json(telefones),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
