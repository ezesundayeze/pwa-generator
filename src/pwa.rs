use clap::Parser;


/// Main command
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Eze Sunday")]
pub struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// Subcommands
#[derive(Parser, Debug)]
enum SubCommand {
    Init(Init),
}

/// Initializes a new PWA
#[derive(Parser, Debug)]
pub struct Init {
    /// Sets the logo URL or path
    #[clap(short, long)]
    logo: Option<String>,
    /// Sets the app name
    #[clap(short, long)]
    name: Option<String>,
    /// Sets the path to static files
    #[clap(short = 's', long)]
    static_files: Option<String>,
    /// Sets the app description
    #[clap(short, long)]
    description: Option<String>,
    /// Sets the background color
    #[clap(short, long)]
    background: Option<String>,
    /// Sets the base URL
    #[clap(short, long)]
    url: Option<String>,
    /// Sets the theme color
    #[clap(short, long)]
    theme: Option<String>,
    /// Sets the icon path
    #[clap(short, long)]
    icon: Option<String>,
}


pub fn init_pwa(args: Init) {
    let logo = get_input(
        args.logo,
        "Logo was not provided. Please enter the logo URL or path:",
    );
    println!("Logo: {}", logo);

    let name = get_input(
        args.name,
        "Name was not provided. Please enter the app name:",
    );
    println!("Name: {}", name);

    let static_files = get_input(
        args.static_files,
        "Static files path was not provided. Please enter the path to static files:",
    );
    println!("Static: {}", static_files);

    let description = get_input(
        args.description,
        "Description was not provided. Please enter the app description:",
    );
    println!("Description: {}", description);

    let background = get_input(
        args.background,
        "Background color was not provided. Please enter the background color:",
    );
    println!("Background: {}", background);

    let url = get_input(args.url, "URL was not provided. Please enter the base URL:");
    println!("URL: {}", url);

    let theme = get_input(
        args.theme,
        "Theme color was not provided. Please enter the theme color:",
    );
    println!("Theme: {}", theme);

    let icon = get_input(
        args.icon,
        "Icon path was not provided. Please enter the icon path:",
    );
    println!("Icon: {}", icon);
}

pub fn get_input(value: Option<String>, message: &str) -> String {
    value.unwrap_or_else(|| {
        println!("{}", message);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    })
}
