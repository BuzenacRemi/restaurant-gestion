use rocket::Request;
use rocket_dyn_templates::{context, Template};

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("hbs/error/404", context! {
        uri: req.uri()
    })
}