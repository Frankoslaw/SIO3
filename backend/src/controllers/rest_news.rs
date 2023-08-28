use crate::services::news::*;
use actix_web::web::Json;
use actix_web::{web, Responder};

pub fn configure<T: 'static + NewsService>(service: web::Data<T>, cfg: &mut web::ServiceConfig) {
    cfg.app_data(service);
    cfg.route("/c/{competition_id}/news", web::get().to(get_news::<T>));
}

async fn get_news<T: NewsService>(
    service: web::Data<T>,
    path: web::Path<(i32,)>,
) -> impl Responder {
    Json(service.get_news(path.into_inner().0).await)
}
