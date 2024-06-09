use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize)]
pub struct APIResponse{
    pub message:String,
}
#[derive(Deserialize,Serialize,Debug)]
pub struct Users{
    pub id: i32,
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub password: Option<String>,
}

#[derive(Serialize,Deserialize)]
pub struct CreateRespore{
    pub email:String,
    pub password:String,

}
#[derive(Serialize,Deserialize)]
pub struct LoginReq{
    pub email:String,
    pub password:String,

}

#[derive(Serialize,Deserialize)]
pub struct LoginResponse{
    pub token: String,
    pub message: String,
}

#[derive(Serialize,Deserialize)]
pub struct EditUserReq{
    pub new_username: String,
    pub new_email: String,
    pub new_phone: i32,
    pub updated_at: Option<NaiveDateTime>,
    pub new_password: String,
}

#[derive(Serialize,Deserialize)]
pub struct GetAllUser{
    pub id:i32,
    pub username: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub is_active:Option<bool>,
    pub sofl_delete:Option<bool>,
}