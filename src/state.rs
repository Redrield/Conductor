use ds::{DriverStation, Mode, Alliance, TcpPacket};
use web_view::{Handle, Error, Content, WebView};
use crate::ipc::Message;
use std::thread;
use std::sync::mpsc;
use std::path::Path;
use std::fs::{self, File, OpenOptions};
use std::io::Write;

pub struct State {
    pub ds: DriverStation,
    mode: Mode,
    extra_stdout_handle: Option<Handle<()>>,
    log_file: String
}

impl State {
    pub fn new(log_file: String) -> State {
        if !Path::new(&log_file).exists() {
            File::create(log_file.clone()).unwrap();
        }
        State {
            ds: DriverStation::new_team(0, Alliance::new_red(1)),
            mode: Mode::Autonomous,
            extra_stdout_handle: None,
            log_file
        }
    }

    fn update_consumer(&mut self, handle: Handle<()>) {
        let mut extra_window_handle = self.extra_stdout_handle.take();
        let mut log_file = OpenOptions::new().write(true).open(&self.log_file).unwrap();
        self.ds.set_tcp_consumer(move |packet| {
            let TcpPacket::Stdout(msg) = packet;
            log_file.write_all(format!("[{:.4}] {}\n", msg.timestamp, msg.message).as_bytes()).unwrap();
            let msg = serde_json::to_string(&Message::NewStdout { message: msg.message }).unwrap();
//            if let Some(handle) = &extra_window_handle {
//                let msg = msg.clone();
//                if let Err(Error::Dispatch) = handle.dispatch(move |wv| wv.eval(&format!("update({})", msg))) {
//                    extra_window_handle = None; // Drop the handle if Dispatch error happens, because that means the window was closed
//                }
//            }
            let _ = handle.dispatch(move |wv| wv.eval(&format!("update({})", msg)));
        });
    }

    pub fn update_ds(&mut self, team_number: u32, handle: Handle<()>) {
        self.ds = DriverStation::new_team(team_number, Alliance::new_red(1));
        self.update_consumer(handle);
        self.ds.set_mode(self.mode);
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
        self.ds.set_mode(self.mode);
    }

    pub fn launch_stdout_window(&mut self, init: Vec<String>, super_handle: Handle<()>) {
        let (tx, rx) = mpsc::channel::<Handle<()>>();
        thread::spawn(move || {
            let wv = web_view::builder()
                .title("Robot Console")
                .size(750, 750)
                .content(Content::Url("http://localhost:8000/stdout"))
                .debug(true)
                .user_data(())
                .invoke_handler(|_, _| { Ok(()) })
                .build().unwrap();

            tx.send(wv.handle());

            wv.run()
        });

        self.extra_stdout_handle = Some(rx.recv().unwrap());
        self.update_consumer(super_handle);
    }
}