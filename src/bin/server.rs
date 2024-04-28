extern crate cr8s;

use dotenv::dotenv;
use rocket::routes;

use rocket_db_pools::Database;

#[rocket::main]
async fn main() {
    dotenv().ok();

    let _ = rocket::build()
        .mount(
            "/",
            routes![
                cr8s::routes::authorization::login,
                cr8s::routes::companies::get_companies,
                cr8s::routes::companies::get_companies_by_id,
                cr8s::routes::companies::create_companies,
                cr8s::routes::companies::update_companies,
                cr8s::routes::companies::delete_companies,
                cr8s::routes::projects::get_projects,
                cr8s::routes::projects::get_projects_by_id,
                cr8s::routes::projects::create_projects,
                cr8s::routes::projects::update_projects,
                cr8s::routes::projects::delete_projects
            ],
        )
        .attach(cr8s::routes::CacheConn::init())
        .attach(cr8s::routes::DbConn::init())
        .launch()
        .await
        .expect("Rocket launch failed");
}
