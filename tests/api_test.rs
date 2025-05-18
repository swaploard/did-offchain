use actix_web::{test, App};
use backend_boilerplate::{routes, models::user::User};

#[actix_web::test]
async fn test_get_users() {
    // Create test app
    let app = test::init_service(
        App::new()
            .service(actix_web::web::scope("/api")
            .configure(routes::user::configure))
    ).await;

    // Create test request
    let req = test::TestRequest::get().uri("/api/user").to_request();

    // Perform the request and verify the response
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Parse the response body
    let users: Vec<User> = test::read_body_json(resp).await;
    assert_eq!(users.len(), 2);
    assert_eq!(users[0].name, "Alice");
    assert_eq!(users[1].name, "Bob");
}