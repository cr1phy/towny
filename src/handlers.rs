use std::str::FromStr;

use actix_web::{get, post, web, HttpResponse};
use sea_orm::prelude::Uuid;
use serde_json::json;

use crate::{
    forms::UserForm,
    services::{mutation::Mutation, query::Query},
    AppState,
};

// TODO: status of api
#[get("/")]
async fn root(data: web::Data<AppState>) -> HttpResponse {
    let online_users = data.online_users.lock().await.to_owned();
    let version = env!("CARGO_PKG_VERSION");
    let body = json! {{ "status": "Ok!", "version": version, "online_users": online_users }};
    HttpResponse::Ok().json(body)
}

// TODO: get user
#[get("/user/{id}")]
async fn get_user(data: web::Data<AppState>, path: web::Path<String>) -> HttpResponse {
    let db = &data.db;

    let id = path.into_inner();
    let id = if let Ok(id) = Uuid::from_str(&id) {
        id
    } else {
        return HttpResponse::BadRequest().finish();
    };

    match Query::get_user_by_id(db, id).await {
        Ok(opt) => match opt {
            Some(user) => HttpResponse::Ok().json(user),
            None => HttpResponse::NotFound().finish(),
        },
        Err(_e) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/user/delete")]
async fn delete_user() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// TODO: create user
#[post("/auth/create")]
async fn create_account(data: web::Data<AppState>, form: web::Json<UserForm>) -> HttpResponse {
    let db = &data.db;
    let form = form.into_inner();

    match Mutation::create_user(db, form).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::BadRequest().json(err.to_string()),
    }
}

#[get("/auth/login")]
async fn login() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/auth/logout")]
async fn logout() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/play")]
async fn play() -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(root)
        .service(create_account)
        .service(get_user)
        .service(delete_user)
        .service(login)
        .service(logout)
        .service(play);
}
