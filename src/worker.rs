use regex::Regex;
use std::fs;

pub fn create(
    root_static_path: String,
    root_html_path: String,
    logo: String,
    js_file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Cache some static files
    let paths = vec![&root_static_path, &root_html_path, &logo];

    // Convert the vector into the JavaScript array format
    let js_array: String = paths
        .iter()
        .map(|path| format!("'{}'", path))
        .collect::<Vec<_>>()
        .join(",\n");

    // Read the JavaScript code from the file
    let js_code =
        fs::read_to_string(js_file_path).expect("Unable to read Javascript values to string");

    // Use a regex to replace the paths section in the js_code
    let re = Regex::new(r"return cache\.addAll\(\[(?s).*?\]\);").unwrap();
    let updated_js = re
        .replace(
            &js_code,
            &format!("return cache.addAll([\n{}\n]);", js_array),
        )
        .to_string();

    // Write the updated JavaScript code to a file
    let worker_path = format!("{}/worker.js", root_static_path);
    fs::write(worker_path, updated_js).expect("Unable to write worker script to a file");

    Ok(())
}
