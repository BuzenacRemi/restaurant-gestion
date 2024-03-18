use rocket::Request;
use rocket_dyn_templates::{context, Template};

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("hbs/error/layout", context! {
        uri: req.uri()
    })
}