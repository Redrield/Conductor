use human_panic::{handle_dump, Metadata};
use std::fmt::Result as FmtResult;
use std::fmt::Write;
use std::panic::PanicInfo;
use std::path::Path;
use tinyfiledialogs::{message_box_ok, MessageBoxIcon};

pub fn hook(info: &PanicInfo) {
    let meta = Metadata {
        version: env!("CARGO_PKG_VERSION").into(),
        name: env!("CARGO_PKG_NAME").into(),
        authors: env!("CARGO_PKG_AUTHORS").replace(":", ", ").into(),
        homepage: env!("CARGO_PKG_HOMEPAGE").into(),
    };

    let file = handle_dump(&meta, info);
    let msg = create_msg(file, &meta).expect("Failed to create error message");
    message_box_ok("An Unexpected Error Occured", &msg, MessageBoxIcon::Error);
}

pub fn create_msg<P: AsRef<Path>>(
    file_path: Option<P>,
    meta: &Metadata,
) -> Result<String, std::fmt::Error> {
    let (_version, name, authors, homepage) =
        (&meta.version, &meta.name, &meta.authors, &meta.homepage);

    let mut buffer = String::new();

    writeln!(&mut buffer, "Well, this is embarrassing.\n")?;
    writeln!(
        &mut buffer,
        "{} had a problem and crashed. To help diagnose the \
     problem you can send us a crash report.\n",
        name
    )?;
    writeln!(
        &mut buffer,
        "A report file has been generated at \"{}\". Submit an \
     issue and include the \
     details in the report file so that I can diagnose the error.\n",
        match file_path {
            Some(fp) => format!("{}", fp.as_ref().display()),
            None => "<Failed to store file to disk>".to_string(),
        }
    )?;

    writeln!(
        &mut buffer,
        "\nConductor does not perform automated error collection. In order to improve the software, I rely on \
     people to submit reports.\n"
    )?;
    writeln!(&mut buffer, "Thank you kindly!")?;

    Ok(buffer)
}
