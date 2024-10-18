use actix_web::{get, post, web, HttpResponse};

// TODO: status of api
#[get("/")]
async fn root() -> HttpResponse {
    HttpResponse::Ok().json("Ok!")
}

#[post("/user")]
async fn create_user() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/user/{id}")]
async fn get_user() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[post("/user/delete")]
async fn delete_user() -> HttpResponse {
    HttpResponse::Ok().finish()
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
        .service(create_user)
        .service(get_user)
        .service(delete_user);
}
