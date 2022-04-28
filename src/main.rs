use axum::{
    http::StatusCode,
    handler::Handler,
    response::{Html, IntoResponse, Response},
    routing::get,
    routing::get_service,
    Router,
};
use tower_http::{services::ServeDir};
use askama::Template;

mod logging; use logging::*;

#[tokio::main]
async fn main() {
    let ip = "127.0.0.0";
    let port = "3000";
    let full = format!("{}:{}", ip, port);
    info(&format!("Starting server with IP and port: {}", full));
    let app = Router::new()
    // Serve static files like CSS
    .nest(
        "/static",
        get_service(ServeDir::new("static/")).handle_error(|error: std::io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unhandled internal error: {}", error),
            )
        }),
    )

    // Routes
    .route("/", get(root));
    let app = app.fallback(h404.into_service());
    axum::Server::bind(&full.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//Route functions
async fn root() -> impl IntoResponse {
    let template = HelloTemplate {};
    HtmlTemplate(template)
}

async fn h404() -> impl IntoResponse {
    let template = Handle404 {};
    HtmlTemplate(template)
}

// Templates
#[derive(Template)]
#[template(path = "index.html")]
struct HelloTemplate {
}

#[derive(Template)]
#[template(path = "404.html")]
struct Handle404 {

}

struct HtmlTemplate<T>(T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error: {}", err),
            ).into_response(),
        }
    }
}