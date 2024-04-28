use std::str::FromStr;

use diesel_async::{AsyncConnection, AsyncPgConnection};

use crate::auth;

use crate::models::RoleCode;
use crate::{
    models::NewUser,
    repositories::{RoleRepositories, UserRepositories},
};

async fn load_db_connection() -> AsyncPgConnection {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    AsyncPgConnection::establish(&database_url)
        .await
        .expect("Error connecting to database")
}

pub async fn create_user(username: String, password: String, role_codes: Vec<String>) {
    let mut c = load_db_connection().await;

    let password_hash = auth::hash_password(password).unwrap();
    let new_user = NewUser {
        username,
        password: password_hash,
    };

    let role_enums = role_codes
        .iter()
        .map(|v| RoleCode::from_str(v.as_str()).unwrap())
        .collect();

    let user = UserRepositories::create(&mut c, new_user, role_enums)
        .await
        .unwrap();

    println!("user: {:?}", user);

    let roles = RoleRepositories::find_by_user(&mut c, &user).await.unwrap();
    println!("roles: {:?}", roles);
}

pub async fn delete_user(id: i32) {
    let mut c = load_db_connection().await;
    UserRepositories::delete(&mut c, id).await.unwrap();
}

pub async fn list_users() {
    let mut c: AsyncPgConnection = load_db_connection().await;

    let users = UserRepositories::find_all_with_role(&mut c).await.unwrap();

    for user in users {
        println!("user: {:?}", user);
    }
}
