use rocket_dyn_templates::Template;
use crate::api::all_api::get_everything;

#[get("/")]
pub async fn index() -> &'static str {
   get_everything().await.to_string().as_str()
}