use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_get_projects() {
    // Setup
    let client = Client::new();
    let companies = common::create_test_companies(&client);
    let a_project = common::create_test_project(&client, &companies);
    let b_project = common::create_test_project(&client, &companies);

    // Test
    let response = client
        .get(format!("{}/projects", common::APP_HOST))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&a_project));
    assert!(json.as_array().unwrap().contains(&b_project));

    // Cleanup
    common::delete_test_project(&client, a_project);
    common::delete_test_project(&client, b_project);
    common::delete_test_company(&client, companies);
}

#[test]
fn test_create_project() {
    // Setup
    let client = Client::new();
    let company = common::create_test_companies(&client);

    // Test
    let response = client
        .post(format!("{}/projects", common::APP_HOST))
        .json(&json!({
            "company_id": company["id"],
            "code": "foo",
            "name": "Foo project",
            "version": "0.1",
            "description": "Foo project description"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    let a_project: Value = response.json().unwrap();
    assert_eq!(
        a_project,
        json!({
            "id": a_project["id"],
            "project_code": "foo",
            "project_name": "Foo project",
            "company_id": company["id"],
        })
    );

    // Cleanup
    common::delete_test_project(&client, a_project);
    common::delete_test_company(&client, company);
}

#[test]
fn test_view_project() {
    // Setup
    let client = Client::new();
    let company = common::create_test_companies(&client);
    let a_project = common::create_test_project(&client, &company);

    // Test
    let response = client
        .get(format!("{}/projects/{}", common::APP_HOST, a_project["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let a_project: Value = response.json().unwrap();
    assert_eq!(
        a_project,
        json!({
            "id": a_project["id"],
            "project_code": "foo",
            "project_name": "Foo project",
            "company_id": company["id"],
        })
    );

    // Cleanup
    common::delete_test_project(&client, a_project);
    common::delete_test_company(&client, company);
}

#[test]
fn test_update_project() {
    // Setup
    let client = Client::new();
    let company = common::create_test_companies(&client);
    let a_project = common::create_test_project(&client, &company);

    // Test
    let response = client
        .put(format!("{}/projects/{}", common::APP_HOST, a_project["id"]))
        .json(&json!({
            "project_code": "foo",
            "project_name": "Foo project",
            "company_id": company["id"],
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let a_project: Value = response.json().unwrap();
    assert_eq!(
        a_project,
        json!({
            "id": a_project["id"],
            "project_code": "foo",
            "project_name": "Foo project",
            "company_id": company["id"],
        })
    );

    // Cleanup
    common::delete_test_project(&client, a_project);
    common::delete_test_company(&client, company);
}

#[test]
fn test_delete_project() {
    // Setup
    let client = Client::new();
    let company = common::create_test_companies(&client);
    let a_project = common::create_test_project(&client, &company);

    // Test
    let response = client
        .delete(format!("{}/projects/{}", common::APP_HOST, a_project["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Cleanup
    common::delete_test_company(&client, company);
}
