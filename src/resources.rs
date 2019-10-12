use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "ui/dist"]
pub struct Resources;