use slint_build::{CompilerConfiguration, EmbedResourcesKind};
use std::path::PathBuf;

fn main() {
    let ui_path = PathBuf::from("ui/app-window.slint");

    let config = CompilerConfiguration::new().embed_resources(EmbedResourcesKind::EmbedFiles);

    slint_build::compile_with_config(ui_path, config).expect("Slint build failed");

    // slint_build::compile("ui/app-window.slint").expect("Slint build failed");
}
