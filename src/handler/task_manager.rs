use crate::model::{request, response};
use crate::service::interface::TaskServiceInterface;
use crate::service::task_manager::TaskService;
// use crate::util::token;

use actix_web::{
    http::header::HeaderMap,
    web::{Data, Json, Path},
    HttpRequest, HttpResponse, Responder,
};
use log::error;
use std::error::Error;
use std::sync::Mutex;

pub async fn get_task(req: HttpRequest, data: Data<Mutex<TaskService>>) -> impl Responder {
    let mut service = data.lock().unwrap();

    let x_ref_id = get_ref_id(req.headers());
    let user_id = match get_user_id(req.headers()) {
        Ok(user_id) => user_id,
        Err(err) => {
            error!("get user_id error: {:}", err);

            return HttpResponse::Unauthorized()
                .insert_header(("x-ref-id", x_ref_id))
                .finish();
        }
    };

    match service.find_all(user_id.to_string()) {
        Ok(result) => {
            let response = response::create_task_response(
                "200",
                "success",
                Some(response::TaskResponseData::Tasks(result)),
            );
            HttpResponse::Ok()
                .insert_header(("x-ref-id", x_ref_id))
                .json(response)
        }
        Err(err) => {
            error!("get task error: {:?}", err);

            let response = response::create_task_response("500", err.to_string().as_str(), None);
            HttpResponse::Ok()
                .insert_header(("x-ref-id", x_ref_id))
                .json(response)
        }
    }
}

pub async fn get_task_by_id(
    req: HttpRequest,
    task_id: Path<String>,
    data: Data<Mutex<TaskService>>,
) -> impl Responder {
    let mut service = data.lock().unwrap();

    let x_ref_id = get_ref_id(req.headers());
    let user_id = match get_user_id(req.headers()) {
        Ok(user_id) => user_id,
        Err(err) => {
            error!("get user_id error: {:?}", err);

            return HttpResponse::Unauthorized()
                .insert_header(("x-ref-id", x_ref_id))
                .finish();
        }
    };

    match service.find_by_id(task_id.to_string(), user_id.to_string()) {
        Ok(result) => {
            let response = response::create_task_response(
                "200",
                "success",
                Some(response::TaskResponseData::Task(result.unwrap())),
            );
            HttpResponse::Ok()
                .insert_header(("x-ref-id", x_ref_id))
                .json(response)
        }
        Err(err) => {
            error!("get task error: {:?}", err);

            let response = response::create_task_response("500", err.to_string().as_str(), None);
            HttpResponse::Ok()
                .insert_header(("x-ref-id", x_ref_id))
                .json(response)
        }
    }
}

pub async fn create_task(
    req: HttpRequest,
    task_request: Json<request::TaskRequest>,
    data: Data<Mutex<TaskService>>,
) -> impl Responder {
    let mut service = data.lock().unwrap();

    let x_ref_id = get_ref_id(req.headers());
    let user_id = match get_user_id(req.headers()) {
        Ok(user_id) => user_id,
        Err(err) => {
            error!("get user_id error: {:?}", err);

            return HttpResponse::Unauthorized()
                .insert_header(("x-ref-id", x_ref_id))
                .finish();
        }
    };

    // service
    match service.insert(task_request.into_inner(), user_id) {
        Ok(result) => {
            let response = response::create_task_response(
                "200",
                "success",
                Some(response::TaskResponseData::Task(result)),
            );
            HttpResponse::Ok()
                .insert_header(("x-ref-id", x_ref_id))
                .json(response)
        }
        Err(err) => {
            error!("get task error: {:?}", err);

            let response = response::create_task_response("500", err.to_string().as_str(), None);
            HttpResponse::Ok()
                .insert_header(("x-ref-id", x_ref_id))
                .json(response)
        }
    }
}

pub async fn update_task_by_id(
    task_id: Path<String>,
    req: HttpRequest,
    task_request: Json<request::TaskRequest>,
    data: Data<Mutex<TaskService>>,
) -> impl Responder {
    let mut service = data.lock().unwrap();

    let x_ref_id = get_ref_id(req.headers());
    let user_id = match get_user_id(req.headers()) {
        Ok(user_id) => user_id,
        Err(err) => {
            error!("get user_id error: {:?}", err);

            return HttpResponse::Unauthorized()
                .insert_header(("x-ref-id", x_ref_id))
                .finish();
        }
    };

    match service.update(
        task_request.into_inner(),
        task_id.to_string(),
        user_id.to_string(),
    ) {
        Ok(result) => {
            let response = response::create_task_response(
                "200",
                "success",
                Some(response::TaskResponseData::Task(result)),
            );
            HttpResponse::Ok()
                .insert_header(("x-ref-id", x_ref_id))
                .json(response)
        }
        Err(err) => {
            error!("get task error: {:?}", err);

            let response = response::create_task_response("500", err.to_string().as_str(), None);
            HttpResponse::Ok()
                .insert_header(("x-ref-id", x_ref_id))
                .json(response)
        }
    }
}

pub async fn delete_task_by_id(
    task_id: Path<String>,
    req: HttpRequest,
    data: Data<Mutex<TaskService>>,
) -> impl Responder {
    let mut service = data.lock().unwrap();

    let x_ref_id = get_ref_id(req.headers());
    let user_id = match get_user_id(req.headers()) {
        Ok(user_id) => user_id,
        Err(err) => {
            error!("get user_id error: {:?}", err);

            return HttpResponse::Unauthorized()
                .insert_header(("x-ref-id", x_ref_id))
                .finish();
        }
    };

    match service.delete(task_id.to_string(), user_id.to_string()) {
        Ok(_) => {
            let response = response::create_task_response("200", "success", None);
            HttpResponse::Ok()
                .insert_header(("x-ref-id", x_ref_id))
                .json(response)
        }
        Err(err) => {
            error!("get task error: {:?}", err);

            let response = response::create_task_response("500", err.to_string().as_str(), None);
            HttpResponse::Ok()
                .insert_header(("x-ref-id", x_ref_id))
                .json(response)
        }
    }
}

fn get_ref_id(header: &HeaderMap) -> String {
    let x_ref_id = header
        .get("x-ref-id")
        .map(|id| id.to_str().unwrap_or_default())
        .unwrap_or(uuid::Uuid::new_v4().to_string().as_str())
        .to_string();

    return x_ref_id;
}

fn get_user_id(header: &HeaderMap) -> Result<String, Box<dyn Error>> {
    // let x_token = header
    //     .get("x_token")
    //     .map(|id| id.to_str().unwrap_or_default())
    //     .unwrap_or_default()
    //     .to_string();

    // // verify token
    // let user_id = token::get_user_id(x_token.as_str())?;

    let user_id = header
        .get("user-id")
        .map(|id| id.to_str().unwrap_or_default())
        .unwrap_or_default()
        .to_string();
    Ok(user_id)
}
