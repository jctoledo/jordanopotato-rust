#[cfg(test)]
mod server_tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        Router,
    };
    use tower::ServiceExt; // for `oneshot` method

    // We'll re-import or replicate the "create_app" logic from main
    // so we can run tests. Alternatively, you can refactor your main
    // to provide a function that returns Router.
    fn create_app() -> Router {
        Router::new()
            .route(
                "/",
                axum::routing::get(|| async { "Hello from Rust + Axum!" }),
            )
            .route("/health", axum::routing::get(|| async { StatusCode::OK }))
    }

    #[tokio::test]
    async fn test_root_endpoint() {
        let app = create_app();

        let request = Request::builder().uri("/").body(Body::empty()).unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body_bytes = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let body_str = std::str::from_utf8(&body_bytes).unwrap();

        assert_eq!(body_str, "Hello from Rust + Axum!");
    }

    #[tokio::test]
    async fn test_health_endpoint() {
        let app = create_app();

        let request = Request::builder()
            .uri("/health")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
