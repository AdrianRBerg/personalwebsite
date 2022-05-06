#[derive(diesel::Queryable)]
struct Post {
    id: i32,
    title: String,
    body: String,
    published: bool,
    date: String,
    lang: String
}
