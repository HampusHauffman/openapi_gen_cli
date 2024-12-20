use clap::{Arg, ArgAction, Command};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::process::Command as ProcessCommand;
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

// Define the default configuration
const DEFAULT_CLIENT_LANGUAGE: &str = "dart-dio";
const DEFAULT_SERVER_LANGUAGE: &str = "kotlin-spring";
const DEFAULT_LOCATION: &str = ".";

struct AppConfig {
    client_language: String,
    server_language: String,
    location: String,
    watch: bool,
}

async fn watch_file(path: &Path, config: &AppConfig) {
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
    watcher.watch(path, RecursiveMode::NonRecursive).unwrap();

    let mut last_event_time = Instant::now();
    let debounce_duration = Duration::from_secs(2); // Adjust debounce duration as needed

    loop {
        match rx.recv() {
            Ok(_) => {
                let now = Instant::now();
                if now.duration_since(last_event_time) >= debounce_duration {
                    println!("File changed, regenerating code...");
                    last_event_time = now;
                    generate_code(config).await;
                }
            }
            Err(e) => println!("Watch error: {:?}", e),
        }
    }
}

async fn generate_code(config: &AppConfig) {
    let output = ProcessCommand::new("docker")
        .args([
            "run",
            "--rm",
            "-v",
            &format!("{}:/local", config.location),
            "openapitools/openapi-generator-cli",
            "generate",
            "-i",
            "/local/openapi.yaml",
            "-g",
            &config.client_language,
            "-o",
            "/local/out/client",
        ])
        .output()
        .expect("Failed to execute process");

    if output.status.success() {
        println!("Client code generation successful!");
    } else {
        eprintln!("Error: {:?}", output);
    }

    let output = ProcessCommand::new("docker")
        .args([
            "run",
            "--rm",
            "-v",
            &format!("{}:/local", config.location),
            "openapitools/openapi-generator-cli",
            "generate",
            "-i",
            "/local/openapi.yaml",
            "-g",
            &config.server_language,
            "-o",
            "/local/out/server",
            "--additional-properties=delegatePattern=true&useSpringBoot3=true",
        ])
        .output()
        .expect("Failed to execute process");

    if output.status.success() {
        println!("Server code generation successful!");
    } else {
        eprintln!("Error: {:?}", output);
    }
}

#[tokio::main]
async fn main() {
    let matches = Command::new("OpenAPI CLI")
        .version("0.1")
        .author("Your Name <your.email@example.com>")
        .about("Generates client and server code from OpenAPI spec")
        .arg(
            Arg::new("watch")
                .short('w')
                .long("watch")
                .help("Watch for changes in the YAML file")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    // Use default configuration values
    let config = AppConfig {
        client_language: DEFAULT_CLIENT_LANGUAGE.to_string(),
        server_language: DEFAULT_SERVER_LANGUAGE.to_string(),
        location: DEFAULT_LOCATION.to_string(),
        watch: matches.get_flag("watch"),
    };

    if config.watch {
        watch_file(Path::new(&config.location), &config).await;
    } else {
        generate_code(&config).await;
    }
}
