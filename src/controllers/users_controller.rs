use crate::helpers::api_error::APIerror;
use crate::middleware::auth::create_jwt;
use crate::models::users_model::{
    CreateRespore, EditUserReq, APIResponse, GetAllUser, LoginReq,
    LoginResponse,
};
use axum::extract::Path;
use axum::{http::StatusCode, Extension, Json};
use axum_macros::debug_handler;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use dotenv::dotenv;
use entity::users;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter, Set,
};
use std::env;
use std::sync::Arc;


async fn db_connection() -> DatabaseConnection {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(db_url).await.unwrap();
    db
}

pub async fn create_user(Json(user_data): Json<CreateRespore>) -> Result<Json<APIResponse>, APIerror> {
    dotenv().ok();
    let conn = db_connection().await;
    let hashpassword = hash(&user_data.password, DEFAULT_COST).map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    });
    let user = users::ActiveModel {
        username: Set(Some(user_data.email.to_owned())),
        email: Set(Some(user_data.email.to_owned())),
        password: Set(Some(hashpassword?.to_owned())),
        updated_at: Set(Some(Utc::now().naive_local())),
        created_at: Set(Some(Utc::now().naive_local())),
        sofl_delete: Set(Some(false)),
        is_active: Set(Some(true)),
        ..Default::default()
    };

    let _ =user.insert(&conn)
        .await
        .map_err(|err| APIerror {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })
        .map(|_model| ());
    
        Ok(Json(APIResponse {
            message: "Create Success".to_string(),
        }))
}

pub async fn login_user(
    Extension(db): Extension<Arc<DatabaseConnection>>,
    Json(user_data): Json<LoginReq>,
) -> Result<Json<LoginResponse>, APIerror> {
    let conn_db = db.as_ref();
    match users::Entity::find()
        .filter(users::Column::Email.eq(user_data.email))
        .one(conn_db)
        .await
    {
        Ok(Some(user)) => {
            if let Some(ref hashed_password) = user.password {
                match verify(&user_data.password, &hashed_password) {
                    Ok(matches) if matches => {
                        match create_jwt(
                            &user.id.to_string(),
                            user.username.as_ref().expect("REASON"),
                        ) {
                            Ok(token) => {
                                let token = LoginResponse {
                                    token,
                                    message: "Login Success".to_string(),
                                };
                                Ok(Json(token))
                            }
                            Err(_) => Err(APIerror {
                                message: "Login False ".to_string(),
                                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                            }),
                        }
                    }
                    Ok(_) => Err(APIerror {
                        message: "Wrong password".to_string(),
                        status_code: StatusCode::NOT_FOUND,
                    }),
                    Err(_) => Err(APIerror {
                        message: "False to verity password".to_string(),
                        status_code: StatusCode::CONFLICT,
                    }),
                }
            } else {
                Err(APIerror {
                    message: "Wrong password".to_owned(),
                    status_code: StatusCode::NOT_FOUND,
                })
            }
        }

        Ok(None) => Err(APIerror {
            message: "user not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
        }),
        Err(_) => Err(APIerror {
            message: "Database error".to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }),
    }
}

#[debug_handler]
pub async fn edit_user(
    Extension(db): Extension<Arc<DatabaseConnection>>,
    Path(id): Path<i32>,
    Json(user_data): Json<EditUserReq>,
) -> Result<Json<APIResponse>, APIerror> {
    let conn_db = db.as_ref();
    let hashpassword = hash(&user_data.new_password, DEFAULT_COST).map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    });
    let mut user: entity::users::ActiveModel = users::Entity::find()
        .filter(users::Column::Id.eq(id))
        .one(conn_db)
        .await
        .map_err(|err| APIerror {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .ok_or_else(|| APIerror {
            message: "User not found".to_string(),
            status_code: StatusCode::NOT_FOUND,
        })?
        .into();

    user.email = Set(Some(user_data.new_email));
    user.username = Set(Some(user_data.new_username));
    user.phone = Set(Some(user_data.new_phone));
    user.updated_at = Set(Some(Utc::now().naive_local()));
    user.password = Set(Some(hashpassword?));

    user.update(conn_db).await.map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;
    Ok(Json(APIResponse {
        message: "Change Success".to_string(),
    }))
}

#[debug_handler]
pub async fn delete_user(
    Extension(db): Extension<Arc<DatabaseConnection>>,
    Path(id): Path<i32>,
) -> Result<Json<APIResponse>, APIerror> {
    let conn_db = db.as_ref();
    let mut user: entity::users::ActiveModel = users::Entity::find()
        .filter(users::Column::Id.eq(id))
        .one(conn_db)
        .await
        .map_err(|err| APIerror {
            message: err.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        })?
        .ok_or_else(|| APIerror {
            message: "User doesn't exists".to_string(),
            status_code: StatusCode::NOT_FOUND,
        })?
        .into();
    user.is_active = Set(Some(false));
    user.sofl_delete = Set(Some(true));

    user.update(conn_db).await.map_err(|err| APIerror {
        message: err.to_string(),
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
    })?;
    Ok(Json(APIResponse {
        message: "Delete Success".to_string(),
    }))
}

pub async fn get_all_user(
    Extension(db): Extension<Arc<DatabaseConnection>>,
) -> Result<Json<Vec<GetAllUser>>, StatusCode> {
    let conn_db = db.as_ref();
    match users::Entity::find().all(conn_db).await {
        Ok(users) => {
            let user_list: Vec<GetAllUser> = users
                .into_iter()
                .map(|user| GetAllUser {
                    id: user.id,
                    username: user.username,
                    email: user.email,
                    phone: user.phone.map(|phonenumer| phonenumer.to_string()),
                    created_at: user.created_at,
                    updated_at: user.updated_at,
                    is_active: user.is_active,
                    sofl_delete: user.sofl_delete,
                })
                .collect();
            Ok(Json(user_list))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
