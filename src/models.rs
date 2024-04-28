use std::io::Write;
use std::str::FromStr;

use crate::schema::*;
use chrono::NaiveDateTime;

use diesel::{deserialize::FromSqlRow, expression::AsExpression, prelude::*};

use diesel::deserialize::FromSql;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::ToSql;

use diesel::sql_types::Text;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Deserialize)]
#[diesel(table_name = companies)]
pub struct Company {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub company_code: Option<String>,
    pub company_name: Option<String>,
    pub photo: Option<String>,
    pub address: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub status: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_deserializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = companies)]
pub struct NewCompany {
    pub company_code: Option<String>,
    pub company_name: Option<String>,
    pub address: Option<String>,
    pub status: Option<String>,
    #[serde(skip_deserializing)]
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Queryable, Serialize, Deserialize, Associations)]
#[diesel(belongs_to(Company))]
#[diesel(table_name = projects)]
pub struct Project {
    #[serde(skip_deserializing)]
    pub id: i32,
    pub project_code: Option<String>,
    pub project_name: Option<String>,
    pub company_id: i32,
    #[serde(skip_deserializing)]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_deserializing)]
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = projects)]
pub struct NewProject {
    pub project_code: Option<String>,
    pub project_name: Option<String>,
    pub company_id: i32,
}

#[derive(Debug, Identifiable, Queryable)]
pub struct Role {
    pub id: i32,
    pub code: RoleCode,
    pub name: String,
    pub create_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub code: RoleCode,
    pub name: String,
}

#[derive(Identifiable, Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub create_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Identifiable, Associations, Queryable, Insertable, Debug)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Role))]
#[diesel(table_name = users_roles)]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Insertable)]
#[diesel(table_name = users_roles)]
pub struct NewUserRole {
    pub user_id: i32,
    pub role_id: i32,
}

#[derive(AsExpression, Debug, FromSqlRow)]
#[diesel(sql_type=Text)]
pub enum RoleCode {
    SuperAdmin,
    Admin,
    Editor,
    Viewer,
    Client,
}

impl ToString for RoleCode {
    fn to_string(&self) -> String {
        match self {
            RoleCode::SuperAdmin => String::from("superadmin"),
            RoleCode::Admin => String::from("admin"),
            RoleCode::Editor => String::from("editor"),
            RoleCode::Viewer => String::from("viewer"),
            RoleCode::Client => String::from("client"),
        }
    }
}

impl FromStr for RoleCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "superadmin" => Ok(RoleCode::SuperAdmin),
            "admin" => Ok(RoleCode::Admin),
            "editor" => Ok(RoleCode::Editor),
            "viewer" => Ok(RoleCode::Viewer),
            "client" => Ok(RoleCode::Client),
            _ => Err(()),
        }
    }
}

impl FromSql<Text, Pg> for RoleCode {
    fn from_sql(value: PgValue<'_>) -> diesel::deserialize::Result<Self> {
        match value.as_bytes() {
            b"superadmin" => Ok(RoleCode::SuperAdmin),
            b"admin" => Ok(RoleCode::Admin),
            b"editor" => Ok(RoleCode::Editor),
            b"viewer" => Ok(RoleCode::Viewer),
            b"client" => Ok(RoleCode::Client),
            _ => Ok(RoleCode::Viewer),
        }
    }
}

impl ToSql<Text, Pg> for RoleCode {
    fn to_sql<'b>(
        &'b self,
        out: &mut diesel::serialize::Output<'b, '_, Pg>,
    ) -> diesel::serialize::Result {
        match self {
            RoleCode::SuperAdmin => out.write_all(b"superadmin")?,
            RoleCode::Admin => out.write_all(b"admin")?,
            RoleCode::Editor => out.write_all(b"editor")?,
            RoleCode::Viewer => out.write_all(b"viewer")?,
            RoleCode::Client => out.write_all(b"client")?,
        };
        Ok(diesel::serialize::IsNull::No)
    }
}
