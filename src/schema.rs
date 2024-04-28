// @generated automatically by Diesel CLI.

diesel::table! {
    companies (id) {
        id -> Int4,
        #[max_length = 255]
        company_code -> Nullable<Varchar>,
        #[max_length = 255]
        company_name -> Nullable<Varchar>,
        #[max_length = 255]
        photo -> Nullable<Varchar>,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        latitude -> Nullable<Float8>,
        longitude -> Nullable<Float8>,
        #[max_length = 255]
        status -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    projects (id) {
        id -> Int4,
        #[max_length = 255]
        project_code -> Nullable<Varchar>,
        #[max_length = 255]
        project_name -> Nullable<Varchar>,
        company_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    roles (id) {
        id -> Int4,
        #[max_length = 64]
        code -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        create_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 128]
        password -> Varchar,
        create_at -> Timestamp,
    }
}

diesel::table! {
    users_roles (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(users_roles -> roles (role_id));
diesel::joinable!(users_roles -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    companies,
    projects,
    roles,
    users,
    users_roles,
);
