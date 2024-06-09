use std::env;

use crate::helpers::api_error::APIerror;
use ::entity::users::Entity;
use axum::{body::Body, extract::Request, http::StatusCode, middleware::Next, response::Response};
use entity::users::Column;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::{ColumnTrait, Database, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    sub: String,
    pub username: String,
}

pub fn create_jwt(id: &str, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(2))
        .expect("valid  timestamp")
        .timestamp();
    let my_secret_key: String = env::var("MY_SECRET_KEY").expect("MY_SECRET_KEY must be set");
    let claims = Claims {
        sub: id.to_owned(),
        exp: exp as usize,
        username: username.to_owned(),
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(my_secret_key.as_ref()),
    )?;
    Ok(token)
}

fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let my_secret_key: String = env::var("MY_SECRET_KEY").expect("MY_SECRET_KEY must be set");
    let validation = Validation::default();
    decode::<Claims>(
        &token,
        &DecodingKey::from_secret(my_secret_key.as_ref()),
        &validation,
    )
    .map(|data| data.claims)
}
pub async fn guards<T>(req: Request<T>, next: Next) -> Result<Response, APIerror> {
    let token = req
        .headers()
        .get("Authorization")
        .ok_or_else(|| {
            let error_message = "No Auth token found".to_owned();
            println!("Error: {}", error_message);
            APIerror {
                message: error_message,
                status_code: StatusCode::BAD_REQUEST,
            }
        })?
        .to_str()
        .map_err(|err| {
            let error_message = format!("Failed to convert token to string. Error: {:?}", err);
            println!("{}", error_message);
            APIerror {
                message: error_message,
                status_code: StatusCode::BAD_REQUEST,
            }
        })?
        .trim();

    if !token.starts_with("Bearer ") {
        let error_message = "Authorization header must start with Bearer".to_owned();
        println!("Error: {}", error_message);
        return Err(APIerror {
            message: error_message,
            status_code: StatusCode::BAD_REQUEST,
        });
    }
    let token = &token[7..];
    let claim = verify_token(token).map_err(|err| {
        println!("Error verifying JWT: {:?}", err);
        APIerror {
            message: "Unauthorized".to_owned(),
            status_code: StatusCode::UNAUTHORIZED,
        }
    })?;

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(db_url).await.unwrap();

    let _identity = Entity::find()
        .filter(Column::Username.eq(claim.username.to_lowercase()))
        .one(&db)
        .await
        .unwrap();

    let req = req.map(|_| Body::empty());
    Ok(next.run(req).await)
}
