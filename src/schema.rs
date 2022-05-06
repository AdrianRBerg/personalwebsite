table! {
    blog_posts (id) {
        id -> Int4,
        title -> Varchar,
        lan -> Bpchar,
        body -> Text,
        published -> Bool,
        date -> Date,
    }
}
