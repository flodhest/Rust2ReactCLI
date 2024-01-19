use dialoguer::Input;
use std::env;
use std::fs;

fn main() {
    // Prompt the user to enter the project name
    let project_name: String = Input::new()
        .with_prompt("Enter the project name")
        .interact()
        .expect("Failed to get project name");

    // Call the function to set up project directories and files
    setup_project_directories_and_files(&project_name);
}

// Function to set up project directories and files
fn setup_project_directories_and_files(project_name: &str) {
    // Get the current directory and join it with the project name
    let react_app_path = env::current_dir()
        .unwrap()
        .join(&project_name)
        .to_string_lossy()
        .to_string();

    // Create necessary folders: Service, Models, components, styles, src
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

    // Read PlaceholderComponent1 content from file
    let placeholder_component1_content = fs::read_to_string("src/PlaceholderComponent1.txt")
        .expect("Failed to read PlaceholderComponent1.txt");

    // Write PlaceholderComponent1 content to file
    fs::write(
        format!(
            "{}/src/components/PlaceholderComponent1.tsx",
            &react_app_path
        ),
        placeholder_component1_content,
    )
    .expect("Failed to add PlaceholderComponent1.tsx");

    // Read PlaceholderComponent2 content from file
    let placeholder_component2_content = fs::read_to_string("src/PlaceholderComponent2.txt")
        .expect("Failed to read PlaceholderComponent2.txt");

    // Write PlaceholderComponent2 content to file
    fs::write(
        format!(
            "{}/src/components/PlaceholderComponent2.tsx",
            &react_app_path
        ),
        placeholder_component2_content,
    )
    .expect("Failed to add PlaceholderComponent2.tsx");

    // Read BackendService boilerplate content
    let backend_service_boilerplate_content = include_str!("backend_service_boilerplate.txt");

    // Write BackendService boilerplate content to file
    fs::write(
        format!("{}/src/Service/BackendService.ts", &react_app_path),
        backend_service_boilerplate_content,
    )
    .expect("Failed to add BackendService.ts");

    // Read PlaceholderModel content from file
    let placeholder_model_content = include_str!("models.txt");

    // Write PlaceholderModel content to file
    fs::write(
        format!("{}/src/Models/PlaceholderModel.tsx", &react_app_path),
        placeholder_model_content,
    )
    .expect("Failed to add PlaceholderModel.tsx");

    // Write development environment file
    fs::write(
        format!("{}/.env.development", &react_app_path),
        "REACT_APP_ENV=development",
    )
    .expect("Failed to create .env.development file");

    // Write production environment file
    fs::write(
        format!("{}/.env.production", &react_app_path),
        "REACT_APP_ENV=production",
    )
    .expect("Failed to create .env.production file");

    // Read PWA service worker content from file
    let pwa_service_worker_content = fs::read_to_string("pwa-service-worker.txt")
        .expect("Failed to read pwa-service-worker.txt");

    // Write PWA service worker content to file
    fs::write(
        format!("{}/src/service-worker.js", &react_app_path),
        pwa_service_worker_content,
    )
    .expect("Failed to add service-worker.js");

    // Read Home component content from file
    let home_component_content = include_str!("home.txt");

    // Write Home component content to file
    fs::write(
        format!("{}/src/components/Home.tsx", &react_app_path),
        home_component_content,
    )
    .expect("Failed to add Home.tsx");

    // Set up styles directory and write main.scss content
    let styles_dir = format!("{}/src/styles", &react_app_path);
    let main_scss_content = include_str!("main_scss.txt");

    fs::write(format!("{}/main.scss", &styles_dir), main_scss_content)
        .expect("Failed to write main.scss");

    // Read App.tsx content from file
    let app_tsx_content = include_str!("app_tsx.txt");

    // Write App.tsx content to file
    fs::write(format!("{}/src/App.tsx", &react_app_path), app_tsx_content)
        .expect("Failed to add App.tsx");

    // Read index.html template content from file
    let index_html_template_content = fs::read_to_string("index_html_template.txt")
        .expect("Failed to read index_html_template.txt");

    // Write index.html template content to file
    fs::write(
        format!("{}/public/index.html", &react_app_path),
        index_html_template_content,
    )
    .expect("Failed to add index.html");

    // Read index.tsx template content from file
    let index_tsx_template_content = fs::read_to_string("index_tsx_template.txt")
        .expect("Failed to read index_tsx_template.txt");

    // Write index.tsx template content to file
    fs::write(
        format!("{}/src/index.tsx", &react_app_path),
        index_tsx_template_content,
    )
    .expect("Failed to add index.tsx");

    // Read manifest.json template content from file
    let manifest_json_template_content = fs::read_to_string("manifest_json_template.txt")
        .expect("Failed to read manifest_json_template.txt");

    // Write manifest.json template content to file
    fs::write(
        format!("{}/public/manifest.json", &react_app_path),
        manifest_json_template_content,
    )
    .expect("Failed to add manifest.json");

    // Read tsconfig.json template content from file
    let tsconfig_json_template_content = fs::read_to_string("tsconfig_json_template.txt")
        .expect("Failed to read tsconfig_json_template.txt");

    // Write tsconfig.json template content to file
    fs::write(
        format!("{}/tsconfig.json", &react_app_path),
        tsconfig_json_template_content,
    )
    .expect("Failed to add tsconfig.json");

    // Read .gitignore template content from file
    let gitignore_template_content = fs::read_to_string("gitignore_template.txt")
        .expect("Failed to read gitignore_template.txt");

    // Write .gitignore template content to file
    fs::write(
        format!("{}/.gitignore", &react_app_path),
        gitignore_template_content,
    )
    .expect("Failed to add .gitignore");

    // Read package.json template content from file
    let package_json_template_content = fs::read_to_string("package_json_template.txt")
        .expect("Failed to read package_json_template.txt");

    // Write package.json template content to file
    fs::write(
        format!("{}/package.json", &react_app_path),
        package_json_template_content,
    )
    .expect("Failed to add package.json");

    println!("Project setup completed successfully!");
}

// Function to create directories based on the provided base path and directory names
fn create_directories(base_path: &str, directories: &[&str]) {
    for dir in directories {
        fs::create_dir_all(format!("{}/{}", base_path, dir))
            .expect(&format!("Failed to create {} directory", dir));
    }
}
