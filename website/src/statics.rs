use std::path::{PathBuf, Path};
use rocket::fs::NamedFile;

#[rocket::get("/assets/<path..>")]
pub async fn second(path: PathBuf) -> Option<NamedFile> {
    let mut path = Path::new(super::relative!("assets")).join(path);
    NamedFile::open(path).await.ok()
}

