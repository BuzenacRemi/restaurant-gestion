use rocket::response::content::RawHtml;
use rocket_dyn_templates::{context, Template};

#[get("/")]
pub fn index() -> Template {
    Template::render("hbs/about/layout",() )
}