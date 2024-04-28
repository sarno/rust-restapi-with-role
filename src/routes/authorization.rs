use crate::routes::{CacheConn, DbConn};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::json;
use rocket::serde::json::Json;
use rocket::serde::json::Value;
use rocket_db_pools::deadpool_redis::redis::AsyncCommands;
use rocket_db_pools::Connection;

use crate::repositories::UserRepositories;

// use crate::routes::DbConn;

use crate::auth::{authorize_user, Credentials};

// use super::CacheConn;

#[rocket::post("/login", format = "json", data = "<creadentials>")]
pub async fn login(
    creadentials: Json<Credentials>,
    mut cache: Connection<CacheConn>,
    mut db: Connection<DbConn>,
) -> Result<Value, Custom<Value>> {
    let user = UserRepositories::find_by_username(&mut db, &creadentials.username)
        .await
        .map_err(|e| Custom(Status::Unauthorized, json!(e.to_string())))?;

    let session_id = authorize_user(&user, creadentials.into_inner())
        .map_err(|_| Custom(Status::Unauthorized, json!("Wrong credentials")))?;

    cache
        .set_ex::<_, _, ()>(format!("sessions/{}", session_id), user.id, 3 * 60 * 60)
        .await
        .map_err(|e| Custom(Status::Unauthorized, json!(e.to_string())))?;

    Ok(json!({
        "token": session_id,
    }))
}
