use dialoguer::{Confirm, Input};
use open;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    let project_name: String = Input::new()
        .with_prompt("Enter the project name")
        .interact()
        .expect("Failed to get project name");

    let installed_node_version = Command::new("node")
        .arg("-v")
        .output()
        .expect("Failed to check Node.js version");

    let installed_node_version_str = String::from_utf8_lossy(&installed_node_version.stdout);

    println!("Installed Node.js version: {}", installed_node_version_str);

    let major_version: Result<u32, _> = installed_node_version_str
        .trim_start_matches('v')
        .split('.')
        .next()
        .ok_or("Failed to extract major version")
        .and_then(|s| s.parse().map_err(|_| "Failed to parse major version"));

    if let Ok(major_version) = major_version {
        println!("Detected Node.js major version: {}", major_version);

        if major_version < 14 || major_version > 15 {
            let download_node_version: bool = Confirm::new()
                .with_prompt(
                    "Do you want to download and install the correct Node.js version (14.x)?",
                )
                .interact()
                .expect("Failed to get user choice for downloading Node.js");

            if download_node_version {
             let _ = open::that("https://nodejs.org/en/download/");

                let mut input = String::new();
                print!("Press Enter when Node.js is installed...");
                io::stdout().flush().unwrap();
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
            } else {
                println!("Please install Node.js version 14.x manually and run the program again.");
                std::process::exit(1);
            }
        } else {
            println!("Node.js version is within the acceptable range.");
        }
    } else {
        println!("Failed to parse Node.js version.");
    }

    setup_project_directories_and_files(&project_name);
}

// Function to set up project directories and files
fn setup_project_directories_and_files(project_name: &str) {
    let react_app_path = env::current_dir()
        .unwrap()
        .join(&project_name)
        .to_string_lossy()
        .to_string();

    // Create folders Service, Models, components, styles, src
    create_directories(
        &react_app_path,
        &[
            "src/Service",
            "src/Models",
            "src/components",
            "src/styles",
            "src",
        ],
    );

    let placeholder_component1_content = fs::read_to_string("src/PlaceholderComponent1.txt")
        .expect("Failed to read PlaceholderComponent1.txt");

    fs::write(
        format!(
            "{}/src/components/PlaceholderComponent1.tsx",
            &react_app_path
        ),
        placeholder_component1_content,
    )
    .expect("Failed to add PlaceholderComponent1.tsx");

    let placeholder_component2_content = fs::read_to_string("src/PlaceholderComponent2.txt")
        .expect("Failed to read PlaceholderComponent2.txt");

    fs::write(
        format!(
            "{}/src/components/PlaceholderComponent2.tsx",
            &react_app_path
        ),
        placeholder_component2_content,
    )
    .expect("Failed to add PlaceholderComponent2.tsx");

    let backend_service_boilerplate_content = include_str!("backend_service_boilerplate.txt");
    fs::write(
        format!("{}/src/Service/BackendService.ts", &react_app_path),
        backend_service_boilerplate_content,
    )
    .expect("Failed to add BackendService.ts");

    let placeholder_model_content = include_str!("models.txt");
    fs::write(
        format!("{}/src/Models/PlaceholderModel.tsx", &react_app_path),
        placeholder_model_content,
    )
    .expect("Failed to add PlaceholderModel.tsx");

    fs::write(
        format!("{}/.env.development", &react_app_path),
        "REACT_APP_ENV=development",
    )
    .expect("Failed to create .env.development file");
    fs::write(
        format!("{}/.env.production", &react_app_path),
        "REACT_APP_ENV=production",
    )
    .expect("Failed to create .env.production file");

    // Read service worker content from text file
    let service_worker_content =
        fs::read_to_string("src/service-worker.txt").expect("Failed to read service_worker.txt");

    // Write service worker code to a file
    fs::write(
        format!("{}/src/service-worker.js", &react_app_path),
        service_worker_content,
    )
    .expect("Failed to add service-worker.js");

    let home_component_content = include_str!("home.txt");

    fs::write(
        format!("{}/src/components/Home.tsx", &react_app_path),
        home_component_content,
    )
    .expect("Failed to add Home.tsx");

    let styles_dir = format!("{}/src/styles", &react_app_path);
    let main_scss_content = include_str!("main_scss.txt");

    fs::write(format!("{}/main.scss", &styles_dir), main_scss_content)
        .expect("Failed to write main.scss");

    let app_tsx_content = include_str!("app_tsx.txt");

    fs::write(format!("{}/src/App.tsx", &react_app_path), app_tsx_content)
        .expect("Failed to add App.tsx");

    println!("Project setup completed successfully!");
}

fn create_directories(base_path: &str, directories: &[&str]) {
    for dir in directories {
        fs::create_dir_all(format!("{}/{}", base_path, dir))
            .expect(&format!("Failed to create {} directory", dir));
    }
}
