use crate::handler::{self};
use actix_web::web;

pub fn config_route(cfg: &mut web::ServiceConfig) {
    cfg.route("/task", web::post().to(handler::task_manager::create_task));
    cfg.route("/task", web::get().to(handler::task_manager::get_task));
    cfg.route(
        "/task/{id}",
        web::get().to(handler::task_manager::get_task_by_id),
    );
    cfg.route(
        "/task/{id}",
        web::put().to(handler::task_manager::update_task_by_id),
    );
    cfg.route(
        "/task/{id}",
        web::delete().to(handler::task_manager::delete_task_by_id),
    );
}
