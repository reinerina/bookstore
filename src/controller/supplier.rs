use crate::service::SupplierService;
use actix_web::{post, web, HttpResponse, Responder};
use mysql_async::Pool;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct SupplierResponse {
    supplier_id: u32,
    name: String,
    telephone: String,
    email: String,
    address: String,
    fax: String,
}

#[post("/supplier/{id}/profile")]
pub async fn supplier_profile(pool: web::Data<Pool>, id: web::Path<(u32,)>) -> impl Responder {
    let supplier_id = id.into_inner().0;
    match pool.get_conn().await {
        Ok(mut conn) => match SupplierService::get_supplier(&mut conn, supplier_id).await {
            Ok(supplier) => HttpResponse::Ok().json(SupplierResponse {
                supplier_id: supplier.id,
                name: supplier.name,
                telephone: supplier.telephone,
                email: supplier.email,
                address: supplier.address,
                fax: supplier.fax,
            }),
            Err(e) => HttpResponse::BadGateway().json(e.to_string()),
        },
        Err(e) => HttpResponse::BadGateway().json(e.to_string()),
    }
}
