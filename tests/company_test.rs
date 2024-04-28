use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_get_companies() {
    let client = Client::new();

    let company1 = common::create_test_companies(&client);
    let company2 = common::create_test_companies(&client);

    // test
    let response = client
        .get(format!("{}/companies", common::APP_HOST))
        .send()
        .unwrap();

    // assert
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&company1));
    assert!(json.as_array().unwrap().contains(&company2));

    // Cleanup
    common::delete_test_company(&client, company1);
    common::delete_test_company(&client, company2);
}

#[test]
fn test_create_company() {
    let client = Client::new();

    let response = client
        .post(format!("{}/companies", common::APP_HOST))
        .json(&json!({
            "company_code": "test",
            "company_name": "test",
            "address": "test",
            "status": "test"
        }))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();

    common::delete_test_company(&client, json);
}

#[test]
fn test_get_companies_by_id() {
    // setup
    let client = Client::new();
    let company = common::create_test_companies(&client);

    let response = client
        .get(format!("{}/companies/{}", common::APP_HOST, company["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    common::delete_test_company(&client, company);
}

#[test]
fn test_update_companies() {
    // setup
    let client = Client::new();

    let company = common::create_test_companies(&client);

    let response = client
        .put(format!("{}/rustaceans/{}", common::APP_HOST, company["id"]))
        .json(&json!({
            "company_code": "test",
            "company_name": "test",
            "address": "test",
            "status": "test"
        }))
        .send()
        .unwrap();
    // assert
    assert_eq!(response.status(), StatusCode::OK);

    common::delete_test_company(&client, company);
}

#[test]
fn test_delete_companies() {
    // setup
    let client = Client::new();

    let company = common::create_test_companies(&client);

    let response = client
        .delete(format!("{}/companies/{}", common::APP_HOST, company["id"]))
        .send()
        .unwrap();

    // assert
    assert_eq!(response.status(), StatusCode::OK);
}
