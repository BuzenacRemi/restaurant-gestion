use rocket::response::content::RawHtml;

#[get("/")]
pub fn index() -> &'static str {
    "About"
}