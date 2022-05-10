use axum::{
    handler::Handler,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::get,
    routing::get_service,
    Extension, Router,
};

use sqlx::postgres::PgPoolOptions;
use sqlx::types::chrono::NaiveDate;
use sqlx::PgPool;

use dotenv::dotenv;
use std::env;

use askama::Template;
use tower_http::services::ServeDir;

mod logging;
use logging::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to SQL database");
    ok("Connected to SQL database");
    let ip = "127.0.0.0";
    let port = "3000";
    let full = format!("{}:{}", ip, port);
    info(&format!("Starting server with IP and port: {}", full));
    let app = Router::new()
        // Serve static files like CSS
        .nest(
            "/static",
            get_service(ServeDir::new("static/")).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        )
        // Routes
        .route("/", get(index))
        .route("/blog", get(blog))
        // SQL
        .layer(Extension(pool));
    let app = app.fallback(h404.into_service());
    axum::Server::bind(&full.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

//Route functions
async fn index() -> impl IntoResponse {
    let template = Index {};
    info("Served HTML at route /");
    HtmlTemplate(template)
}

async fn blog(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let rows = sqlx::query!("SELECT id, title, lang, short_desc, date FROM blog_posts")
        .fetch_all(&pool)
        .await
        .unwrap();
    let mut eng_vec: Vec<BlogList> = Vec::new();
    let mut nor_vec: Vec<BlogList> = Vec::new();

    for row in rows {
        let post = BlogList {
            id: row.id,
            title: row.title.to_string(),
            description: row.short_desc.to_string(),
            date: row.date,
        };

        if row.lang == "en" {
            eng_vec.push(post);
        } else {
            nor_vec.push(post);
        }
    }
    let template = BlogCollection {
        english: eng_vec,
        norwegian: nor_vec,
    };
    info("Served HTML at route /blog");
    HtmlTemplate(template)
}

async fn h404() -> impl IntoResponse {
    let template = Handle404 {};
    HtmlTemplate(template)
}

// Templates
#[derive(Template)]
#[template(path = "index.html")]
struct Index {}

#[derive(Template)]
#[template(path = "blog.html")]
struct BlogCollection {
    english: Vec<BlogList>,
    norwegian: Vec<BlogList>,
}

#[derive(Template)]
#[template(path = "404.html")]
struct Handle404 {}

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
            )
                .into_response(),
        }
    }
}

#[derive(Debug)]
struct BlogList {
    id: i16,
    title: String,
    description: String,
    date: NaiveDate,
}
#[derive(Debug)]
struct BlogPost {
    title: String,
    body: String,
    date: NaiveDate,
}
