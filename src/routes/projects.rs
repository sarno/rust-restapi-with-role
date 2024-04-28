use crate::models::{NewProject, Project, User};
use crate::repositories::ProjectRepositories;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::response::status::NoContent;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;

use crate::routes::DbConn;
use rocket_db_pools::Connection;

#[get("/projects")]
pub async fn get_projects(mut db: Connection<DbConn>, _user: User) -> Result<Value, Custom<Value>> {
    match ProjectRepositories::find_multiple(&mut db, 10).await {
        Ok(projects) => Ok(json!(projects)),
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

#[get("/projects/<id>")]
pub async fn get_projects_by_id(
    mut db: Connection<DbConn>,
    id: i32,
    _user: User,
) -> Result<Value, Custom<Value>> {
    ProjectRepositories::find(&mut db, id)
        .await
        .map(|project| json!(project))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[post("/projects", format = "json", data = "<new_project>")]
pub async fn create_projects(
    mut db: Connection<DbConn>,
    new_project: Json<NewProject>,
    _user: User,
) -> Result<Custom<Json<Value>>, Custom<Value>> {
    match ProjectRepositories::create(&mut db, new_project.into_inner()).await {
        Ok(project) => {
            let response = json!(project);
            Ok(Custom(Status::Created, Json(response)))
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

#[put("/projects/<id>", format = "json", data = "<project>")]
pub async fn update_projects(
    mut db: Connection<DbConn>,
    id: i32,
    project: Json<Project>,
    _user: User,
) -> Result<Value, Custom<Value>> {
    ProjectRepositories::update(&mut db, id, project.into_inner())
        .await
        .map(|project| json!(project))
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}

#[delete("/projects/<id>")]
pub async fn delete_projects(
    mut db: Connection<DbConn>,
    id: i32,
    _user: User,
) -> Result<NoContent, Custom<Value>> {
    ProjectRepositories::delete(&mut db, id)
        .await
        .map(|_| NoContent)
        .map_err(|_| Custom(Status::InternalServerError, json!("Error")))
}
