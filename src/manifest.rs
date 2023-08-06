use serde::{Deserialize, Serialize};
extern crate serde_json;
use std::fs::File;
use std::io::Result;
use std::io::Write;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Icon {
    src: String,
    sizes: String,
    #[serde(rename = "type")]
    icon_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AppInfo {
    name: String,
    short_name: String,
    description: String,
    start_url: Option<String>,
    display: String,
    background_color: String,
    theme_color: String,
    icons: Vec<Icon>,
}

pub fn create(
    name: String,
    short_name: String,
    description: String,
    icon_path: String,
    root_path: String
) -> Result<()> {
    let icon_path = icon_path;
    let extension = Path::new(&icon_path)
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");
    let icon_type = format!("image/{}", extension);

    let app_info = AppInfo {
        name: String::from(name),
        short_name: String::from(short_name),
        description: String::from(description),
        start_url: Some(String::from("/")),
        display: String::from("standalone"),
        background_color: String::from("#f4f4f4"),
        theme_color: String::from("#007bff"),
        icons: vec![Icon {
            src: String::from(icon_path),
            sizes: String::from("192x192"),
            icon_type,
        }],
    };

    let json =
        serde_json::to_string_pretty(&app_info).expect("Failed to serialize app info to JSON");
    let manifest_path =  format!("{}/manifest.json",root_path);
    let mut file = File::create(manifest_path).expect("Failed to create file");

    file.write_all(json.as_bytes())
        .expect("Failed to write to file");

    Ok(())
}
