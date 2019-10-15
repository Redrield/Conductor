use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "web"]
pub struct Resources;