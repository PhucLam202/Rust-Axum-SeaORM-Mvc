use std::env;
use dotenv::dotenv;


pub async fn conn_db()->String{
    dotenv().ok();
    env::var("DATABASE_URL").expect("Data Must Be Set")
}