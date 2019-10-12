use std::thread;
use std::path::PathBuf;
use rocket::*;
use rocket::response::{content, Responder};
use rocket::http::{Status, ContentType};
use crate::resources;

/// React is so fucking pedantic i want to die
struct Fml {
    content: String,
    extension: String
}

impl<'r> Responder<'r> for Fml {
    fn respond_to(self, request: &Request) -> Result<Response<'r>, Status> {
        match self.extension.as_str() {
            "css" => content::Css(self.content).respond_to(request),
            "js" => content::JavaScript(self.content).respond_to(request),
            "svg" => content::Content(ContentType::SVG, self.content).respond_to(request),
            _ => content::Plain(self.content).respond_to(request),
        }
    }
}

pub fn launch_rocket() {
    thread::spawn(|| {
        rocket::ignite().mount("/", routes![static_resources, index]).launch();
    });
}

#[get("/")]
fn index() -> content::Html<String> {
    content::Html(unsafe { String::from_utf8_unchecked(resources::Resources::get("index.html").unwrap().into_owned()) })
}

#[get("/<file..>")]
fn static_resources(file: PathBuf) -> Option<Fml> {
    let ext = file.extension().unwrap().to_os_string().into_string().unwrap();
    let stuff = resources::Resources::get(file.into_os_string().to_str().unwrap()).map(|cow| unsafe { String::from_utf8_unchecked(cow.into_owned()) })?;

    Some(Fml {
        content: stuff,
        extension: ext
    })
}