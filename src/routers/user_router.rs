use axum::{
    middleware, routing::{get, post}, Extension, Router
};
use sea_orm::DatabaseConnection;
use crate::{controllers::users_controller::{
    create_user, delete_user, edit_user, get_all_user, login_user,
}, middleware::auth::guards};
use std::sync::Arc;


//add extension to match with controller can check sync connect database
pub fn user_router(db : Extension<Arc<DatabaseConnection>>) -> Router {
    let pub_router = Router::new()
        .route("/v1/api/create_user", post(create_user).layer(db.clone()))
        .route("/v1/api/login_user", post(login_user).layer(db.clone()));
    //need verify JWT token to login
    let priv_router = Router::new()
        .route("/v1/api/update_user/:id", post(edit_user))
        .route("/v1/api/delete_user/:id", post(delete_user))
        .route("/v1/api/get_all_user", get(get_all_user))
        .layer(middleware::from_fn(guards));
    Router::new().merge(pub_router).merge(priv_router)
}
