#![feature(decl_macro, proc_macro_hygiene)]
use web_view::{Content, WVResult};

mod resources;
use resources::Resources;

mod ws;

fn main() -> WVResult {
    ws::launch_rocket();

    web_view::builder()
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
        .build()?
        .run()
}

