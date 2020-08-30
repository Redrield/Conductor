use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "ui/main-window/build"]
pub struct Resources;

#[derive(RustEmbed)]
#[folder = "ui/stdout/build"]
pub struct StdoutResources;
