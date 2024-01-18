use dialoguer::Input;
use std::env;
use std::fs;

fn main() {
    let project_name: String = Input::new()
        .with_prompt("Enter the project name")
        .interact()
        .expect("Failed to get project name");

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

    let placeholder_component1_content =
        fs::read_to_string("src/PlaceholderComponent1.txt").expect("Failed to read PlaceholderComponent1.txt");

    fs::write(
        format!("{}/src/components/PlaceholderComponent1.tsx", &react_app_path),
        placeholder_component1_content,
    )
    .expect("Failed to add PlaceholderComponent1.tsx");

    let placeholder_component2_content =
        fs::read_to_string("src/PlaceholderComponent2.txt").expect("Failed to read PlaceholderComponent2.txt");

    fs::write(
        format!("{}/src/components/PlaceholderComponent2.tsx", &react_app_path),
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

    fs::write(format!("{}/.env.development", &react_app_path), "REACT_APP_ENV=development")
        .expect("Failed to create .env.development file");
    fs::write(format!("{}/.env.production", &react_app_path), "REACT_APP_ENV=production")
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
