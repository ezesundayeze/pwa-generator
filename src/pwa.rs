use crate::{html_parser, manifest, worker};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Eze Sunday")]
pub struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Init(Init),
}

#[derive(Parser, Debug)]
pub struct Init {
    #[clap(short, long)]
    root_path: Option<String>,
    #[clap(short, long)]
    file_path: Option<String>,
    // Sets the logo URL or path
    #[clap(short, long)]
    logo: Option<String>,
    // Sets the app name
    #[clap(short, long)]
    name: Option<String>,
    // Sets the App name displayed on the home screen
    #[clap(short, long)]
    short_name: Option<String>,
    // Sets the app description
    #[clap(short, long)]
    description: Option<String>,
    // Sets the icon path
    #[clap(short, long)]
    icon: Option<String>,
}

pub fn init_pwa(args: Init) {
    let root_path: String = get_input(
        args.root_path,
        "Enter the path to your static files directory",
        "./example/public/",
    );
    let html_file_path = get_input(
        args.file_path,
        "Enter the path to your root html file, e.g 'index.html':",
        "./example/public/index.html",
    );

    let name = get_input(
        args.name,
        "Enter the app name e.g Relay Wash:",
        "Example App",
    );

    let short_name = get_input(
        args.short_name,
        "Enter the app short name, this is the name that will be on phones home screen. E.g Relay",
        "Example",
    );

    let description = get_input(
        args.description,
        "Please enter the app description:",
        "This is the app description",
    );

    let icon = get_input(
        args.icon,
        "Please enter the path to your app icon:",
        "/images/icon.png",
    );

    let worker_script = format!("{}/worker.js", root_path);
    match html_parser::html_parser(&html_file_path, worker_script.as_str()) {
        Ok(_) => println!("HTML parsing and modification successful."),
        Err(err) => eprintln!(
            "An error occurred during HTML parsing and modification: {}",
            err
        ),
    }

    let worker_jscode = worker::create(
        root_path.clone(),
        html_file_path,
        icon.clone(),
        &String::from("./template/worker.js"),
    );
    match worker_jscode {
        Ok(()) => println!("Worker JS code written"),
        Err(err) => println!("Worker Script: {}", err),
    }

    let manifest_result = manifest::create(name, short_name, description, icon, root_path);
    match manifest_result {
        Ok(()) => println!("Done"),
        Err(err) => println!("{}", err),
    }
}

pub fn get_input(value: Option<String>, message: &str, default_value: &str) -> String {
    value.unwrap_or_else(|| {
        println!("{}", message);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let trimmed_input = input.trim().to_string();
        if trimmed_input.is_empty() {
            println!("Using default value: {}", default_value);
            return default_value.to_string();
        }
        trimmed_input
    })
}
