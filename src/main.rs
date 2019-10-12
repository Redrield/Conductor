#![feature(decl_macro, proc_macro_hygiene)]
use web_view::{Content, WVResult};

mod resources;
use resources::Resources;

mod ws;
mod ipc;

fn main() -> WVResult {
    ws::launch_rocket();

    let mut webview = web_view::builder()
        .title("React in rust native?")
        .content(Content::Url("http://localhost:8000"))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_wv, arg| {
            match arg {
                "test" => println!("Yonk n chonk"),
                _ => {}
            }
            Ok(())
        })
        .build()?;

    // run() copied from web-view so we can put in some refresh calls to update the frontend state with each new tick
    loop {
        match webview.step() {
            Some(Ok(_)) => continue,
            Some(e) => e?,
            None => return Ok(webview.user_data().clone())
        }
    }
}

