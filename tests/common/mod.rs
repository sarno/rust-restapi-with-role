use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub static APP_HOST: &'static str = "http://127.0.0.1:8000";

pub fn create_test_companies(client: &Client) -> Value {
    let response = client
        .post(format!("{}/companies", APP_HOST))
        .json(&json!({
            "company_code": "test",
            "company_name": "test",
            "address": "test",
            "status": "test"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

pub fn create_test_project(client: &Client, project: &Value) -> Value {
    let response = client
        .post(format!("{}/projects", APP_HOST))
        .json(&json!({
            "company_id": project["id"],
            "project_code": "foo",
            "project_name": "Foo crate"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

pub fn delete_test_company(client: &Client, company: Value) {
    let response = client
        .delete(format!("{}/companies/{}", APP_HOST, company["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

pub fn delete_test_project(client: &Client, a_project: Value) {
    let response = client
        .delete(format!("{}/projects/{}", APP_HOST, a_project["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
