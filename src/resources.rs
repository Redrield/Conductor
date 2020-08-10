use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "ui-react/build"]
pub struct Resources;