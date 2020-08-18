use std::thread;
use crate::resources;
use std::sync::{mpsc, RwLock, Arc, Mutex};
use actix_web::{HttpServer, web, HttpRequest, HttpResponse, App};
use actix_web::body::Body;
use actix_web_actors::ws;
use resources::*;
use std::borrow::Cow;
use crate::state::State;
use actix_web::web::Payload;
use actix::Addr;

mod sock;
pub use sock::*;
use actix_web::middleware::Logger;

fn assets(path: web::Path<String>) -> HttpResponse {
    use log::info;

    let path = path.into_inner();
    info!("Path is {}", path);
    match Resources::get(&path) {
        Some(content) => {
            let body: Body = match content {
                Cow::Borrowed(bytes) => bytes.into(),
                Cow::Owned(bytes) => bytes.into(),
            };
            HttpResponse::Ok().content_type(mime_guess::from_path(path).first_or_octet_stream().as_ref()).body(body)
        }
        None => match StdoutResources::get(&path) {
            Some(content) => {
                let body: Body = match content {
                    Cow::Borrowed(bytes) => bytes.into(),
                    Cow::Owned(bytes) => bytes.into(),
                };
                HttpResponse::Ok().content_type(mime_guess::from_path(path).first_or_octet_stream().as_ref()).body(body)
            }
            None => HttpResponse::NotFound().body("404 Not Found")
        }
    }
}

fn index(_req: HttpRequest) -> HttpResponse {
    let contents = Resources::get("index.html").unwrap();
    let body: Body = match contents {
        Cow::Borrowed(bytes) => bytes.into(),
        Cow::Owned(bytes) => bytes.into(),
    };

    HttpResponse::Ok().content_type("text/html")
        .body(body)
}

fn stdout(_req: HttpRequest) -> HttpResponse {
    let contents = StdoutResources::get("index.html").unwrap();
    let body: Body = match contents {
        Cow::Borrowed(bytes) => bytes.into(),
        Cow::Owned(bytes) => bytes.into(),
    };

    HttpResponse::Ok().content_type("text/html")
        .body(body)
}

fn main_sock(req: HttpRequest, stream: Payload) -> HttpResponse {
    // Get app state things
    let state = req.app_data::<Arc<RwLock<State>>>().unwrap();
    let tx = req.app_data::<Mutex<mpsc::Sender<Addr<WebsocketHandler>>>>().unwrap();
    // Start the websocket connection, get the address of the actor
    let (addr, res) = ws::start_with_addr(WebsocketHandler::new(state.clone()), &req, stream).unwrap();
    // Notify the main app of the websocket Addr to be able to send messages to the frontend
    tx.lock().unwrap().send(addr).unwrap();
    res
}

fn stdout_sock(req: HttpRequest, stream: Payload) -> HttpResponse {
    let tx = req.app_data::<Mutex<mpsc::Sender<Addr<StdoutHandler>>>>().unwrap();
    let (addr, res) = ws::start_with_addr(StdoutHandler, &req, stream).unwrap();
    tx.lock().unwrap().send(addr).unwrap();
    res
}

pub fn launch_webserver(state: Arc<RwLock<State>>, addr_sender: mpsc::Sender<Addr<WebsocketHandler>>, stdout_sender: mpsc::Sender<Addr<StdoutHandler>>) -> u16 {
    let (port_tx, port_rx) = mpsc::channel();

    thread::spawn(move || {
        let _= actix_rt::System::new("conductords-actix");

        let server = HttpServer::new(move || {
            // Love redundant cloning to abide by Fn limitations
            App::new()
                .app_data(state.clone())
                .app_data(Mutex::new(addr_sender.clone()))
                .app_data(Mutex::new(stdout_sender.clone()))
                .wrap(Logger::default())
                .route("/", web::get().to(index))
                .route("/stdout", web::get().to(stdout))
                .route("/ws/index", web::get().to(main_sock))
                .route("/ws/stdout", web::get().to(stdout_sock))
                .route("/{path:.*}", web::get().to(assets))
        })
            .bind("127.0.0.1:0")
            .unwrap();

        let port = server.addrs().first().unwrap().port();
        port_tx.send(port).unwrap();

        let server = server.run();
        futures::executor::block_on(server.stop(true));
    });

    port_rx.recv().unwrap()
}
