use actix_web::web;
use crate::handlers::{health, user};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(health::health_check)
        .service(user::create_user)
        .service(user::get_user)
        .service(user::get_all_users)
        .service(user::update_user)
        .service(user::delete_user);
}
