use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};

use crate::models::*;
use crate::schema::*;

pub struct CompanyRepositories;

impl CompanyRepositories {
    pub async fn find(c: &mut AsyncPgConnection, _id: i32) -> QueryResult<Company> {
        companies::table.find(_id).get_result(c).await
    }

    pub async fn find_multiple(c: &mut AsyncPgConnection, limit: i64) -> QueryResult<Vec<Company>> {
        companies::table.limit(limit).load(c).await
    }

    pub async fn delete(c: &mut AsyncPgConnection, _id: i32) -> QueryResult<usize> {
        diesel::delete(companies::table.find(_id)).execute(c).await
    }

    pub async fn create(
        c: &mut AsyncPgConnection,
        new_company: NewCompany,
    ) -> QueryResult<Company> {
        diesel::insert_into(companies::table)
            .values(&new_company)
            .get_result(c)
            .await
    }

    pub async fn update(
        c: &mut AsyncPgConnection,
        _id: i32,
        updated_company: Company,
    ) -> QueryResult<Company> {
        diesel::update(companies::table.find(_id))
            .set((
                companies::company_code.eq(&updated_company.company_code),
                companies::company_name.eq(&updated_company.company_name),
                companies::address.eq(&updated_company.address),
                companies::status.eq(&updated_company.status),
            ))
            .get_result(c)
            .await
    }
}

pub struct ProjectRepositories;

impl ProjectRepositories {
    pub async fn find(c: &mut AsyncPgConnection, _id: i32) -> QueryResult<Project> {
        projects::table.find(_id).get_result(c).await
    }

    pub async fn find_multiple(
        c: &mut AsyncPgConnection,
        _limit: i64,
    ) -> QueryResult<Vec<Project>> {
        projects::table.limit(_limit).load(c).await
    }

    pub async fn create(
        c: &mut AsyncPgConnection,
        new_project: NewProject,
    ) -> QueryResult<Project> {
        diesel::insert_into(projects::table)
            .values(new_project)
            .get_result(c)
            .await
    }

    pub async fn update(
        c: &mut AsyncPgConnection,
        _id: i32,
        a_project: Project,
    ) -> QueryResult<Project> {
        diesel::update(projects::table.find(_id))
            .set((
                projects::id.eq(a_project.id),
                projects::project_code.eq(a_project.project_code),
                projects::project_name.eq(a_project.project_name),
                projects::company_id.eq(a_project.company_id),
            ))
            .get_result(c)
            .await
    }

    pub async fn delete(c: &mut AsyncPgConnection, _id: i32) -> QueryResult<usize> {
        diesel::delete(projects::table.find(_id)).execute(c).await
    }
}

pub struct UserRepositories;

impl UserRepositories {
    pub async fn create(
        c: &mut AsyncPgConnection,
        _new_user: NewUser,
        role_codes: Vec<RoleCode>,
    ) -> QueryResult<User> {
        let user = diesel::insert_into(users::table)
            .values(_new_user)
            .get_result::<User>(c)
            .await
            .unwrap();

        // TODO: add role
        for role_code in role_codes {
            let new_user_role = {
                if let Ok(role) = RoleRepositories::find_by_code(c, &role_code).await {
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                } else {
                    let name = role_code.to_string();
                    let new_role = NewRole {
                        code: role_code,
                        name,
                    };

                    let role = RoleRepositories::create(c, new_role).await.unwrap();
                    NewUserRole {
                        user_id: user.id,
                        role_id: role.id,
                    }
                }
            };

            diesel::insert_into(users_roles::table)
                .values(new_user_role)
                .execute(c)
                .await
                .unwrap();
        }

        Ok(user)
    }

    //show list user
    // pub async fn find_all(c: &mut AsyncPgConnection) -> QueryResult<Vec<User>> {
    //     users::table.load::<User>(c).await
    // }

    pub async fn find_all_with_role(
        c: &mut AsyncPgConnection,
    ) -> QueryResult<Vec<(User, Vec<(UserRole, Role)>)>> {
        let users = users::table.load::<User>(c).await?;
        let result = users_roles::table
            .inner_join(roles::table)
            .load::<(UserRole, Role)>(c)
            .await?
            .grouped_by(&users);

        Ok(users.into_iter().zip(result).collect())
    }

    // delete user
    pub async fn delete(c: &mut AsyncPgConnection, id: i32) -> QueryResult<usize> {
        diesel::delete(users_roles::table.filter(users_roles::user_id.eq(id)))
            .execute(c)
            .await?;

        diesel::delete(users::table.find(id)).execute(c).await
    }

    pub async fn find(c: &mut AsyncPgConnection, id: i32) -> QueryResult<User> {
        users::table.find(id).get_result(c).await
    }

    pub async fn find_by_username(
        c: &mut AsyncPgConnection,
        username: &String,
    ) -> QueryResult<User> {
        users::table
            .filter(users::username.eq(username))
            .get_result(c)
            .await
    }
}

pub struct RoleRepositories;

impl RoleRepositories {
    pub async fn find_by_ids(c: &mut AsyncPgConnection, ids: Vec<i32>) -> QueryResult<Vec<Role>> {
        roles::table.filter(roles::id.eq_any(ids)).load(c).await
    }

    pub async fn find_by_code(c: &mut AsyncPgConnection, code: &RoleCode) -> QueryResult<Role> {
        roles::table.filter(roles::code.eq(code)).first(c).await
    }

    pub async fn find_by_user(c: &mut AsyncPgConnection, user: &User) -> QueryResult<Vec<Role>> {
        let users_roles = UserRole::belonging_to(&user)
            .get_results::<UserRole>(c)
            .await?;

        let role_ids = users_roles.iter().map(|ur: &UserRole| ur.role_id).collect();

        Self::find_by_ids(c, role_ids).await
    }

    pub async fn create(c: &mut AsyncPgConnection, _new_role: NewRole) -> QueryResult<Role> {
        diesel::insert_into(roles::table)
            .values(_new_role)
            .get_result(c)
            .await
    }
}
