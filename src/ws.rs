use std::thread;
use std::path::PathBuf;
use rocket::*;
use rocket::response::{content, Responder};
use rocket::http::{Status, ContentType};
use crate::resources;

/// Contents of some file and its extension, acts as NamedFile without needing std fs apis
struct FileResource {
    content: String,
    extension: String
}

impl<'r> Responder<'r> for FileResource {
    fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
        if let Some(ct) = ContentType::from_extension(&self.extension) {
            content::Content(ct, self.content).respond_to(request)
        }else {
            content::Plain(self.content).respond_to(request)
        }
    }
}

pub fn launch_rocket() {
    thread::spawn(|| {
        rocket::ignite().mount("/", routes![static_resources, index, stdout]).launch();
    });
}

#[get("/")]
fn index() -> content::Html<String> {
    content::Html(unsafe { String::from_utf8_unchecked(resources::Resources::get("index.html").unwrap().into_owned()) })
}

#[get("/stdout")]
fn stdout() -> content::Html<String> {
    content::Html(unsafe { String::from_utf8_unchecked(resources::Resources::get("stdout.html").unwrap().into_owned()) })
}

#[get("/<file..>")]
fn static_resources(file: PathBuf) -> Option<FileResource> {
    let ext = file.extension().unwrap().to_os_string().into_string().unwrap();
    let stuff = resources::Resources::get(file.into_os_string().to_str().unwrap()).map(|cow| unsafe { String::from_utf8_unchecked(cow.into_owned()) })?;

    Some(FileResource {
        content: stuff,
        extension: ext
    })
}