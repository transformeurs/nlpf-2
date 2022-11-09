// `&'static str` becomes a `200 OK` with `content-type: text/plain; charset=utf-8`
pub async fn plain_text() -> &'static str {
    "foo"
}
