use crate::models::{Company, NewCompany};
use crate::repositories::CompanyRepositories;
use crate::routes::DbConn;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::response::status::NoContent;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;

use rocket_db_pools::Connection;

use super::EditorUser;

#[get("/companies")]
pub async fn get_companies(
    mut db: Connection<DbConn>,
    _user: EditorUser,
) -> Result<Custom<Json<Value>>, Custom<Value>> {
    match CompanyRepositories::find_multiple(&mut db, 10).await {
        Ok(companies) => {
            let response = json!(companies);
            Ok(Custom(Status::Ok, Json(response)))
        }
        Err(err) => {
            let error_message = format!("Error: {}", err);
            error!("{}", error_message);
            Err(Custom(
                Status::InternalServerError,
                json!({ "error": error_message }),
            ))
        }
    }
}

#[get("/companies/<id>")]
pub async fn get_companies_by_id(
    mut db: Connection<DbConn>,
    id: i32,
    _user: EditorUser,
) -> Result<Custom<Json<Value>>, Custom<Value>> {
    CompanyRepositories::find(&mut db, id)
        .await
        .map(|company| {
            let response = json!(company);
            Custom(Status::Ok, Json(response))
        })
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[post("/companies", format = "json", data = "<new_company>")]
pub async fn create_companies(
    mut db: Connection<DbConn>,
    new_company: Json<NewCompany>,
    _user: EditorUser,
) -> Result<Custom<Value>, Custom<Value>> {
    CompanyRepositories::create(&mut db, new_company.into_inner())
        .await
        .map(|company| Custom(Status::Created, json!(company)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[put("/companies/<id>", format = "json", data = "<company>")]
pub async fn update_companies(
    mut db: Connection<DbConn>,
    company: Json<Company>,
    id: i32,
    _user: EditorUser,
) -> Result<Custom<Value>, Custom<Value>> {
    CompanyRepositories::update(&mut db, id, company.into_inner())
        .await
        .map(|company| Custom(Status::Ok, json!(company)))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[delete("/companies/<id>")]
pub async fn delete_companies(
    mut db: Connection<DbConn>,
    id: i32,
    _user: EditorUser,
) -> Result<Custom<NoContent>, Custom<Value>> {
    CompanyRepositories::delete(&mut db, id)
        .await
        .map(|_| Custom(Status::Ok, NoContent))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}
