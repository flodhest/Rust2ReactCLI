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

    // Create necessary folders: Service, Models, components, styles, src, public
    create_directories(
        &react_app_path,
        &[
            "src/Service",
            "src/Models",
            "src/components",
            "src/styles",
            "src",
            "public",
        ],
    );

    // Read and write PlaceholderComponent1 content
    copy_template_file(
        "src/PlaceholderComponent1.txt",
        &react_app_path,
        "src/components/PlaceholderComponent1.tsx",
    );

    // Read and write PlaceholderComponent2 content
    copy_template_file(
        "src/PlaceholderComponent2.txt",
        &react_app_path,
        "src/components/PlaceholderComponent2.tsx",
    );

    // Read and write BackendService boilerplate content
    copy_template_file(
        "src/backend_service_boilerplate.txt",
        &react_app_path,
        "src/Service/BackendService.ts",
    );

    // Read and write PlaceholderModel content
    copy_template_file(
        "src/models.txt",
        &react_app_path,
        "src/Models/PlaceholderModel.tsx",
    );

    // Write development and production environment files
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

    // Read and write PWA service worker content
    copy_template_file(
        "src/service-worker.txt",
        &react_app_path,
        "public/service-worker.js",
    );

    // Read and write Home component content
    copy_template_file("src/home.txt", &react_app_path, "src/components/Home.tsx");

    // Set up styles directory and write main.scss content
    let styles_dir = format!("{}/src/styles", &react_app_path);
    let main_scss_content = include_str!("main_scss.txt");
    fs::write(format!("{}/main.scss", &styles_dir), main_scss_content)
        .expect("Failed to write main.scss");
    
    // Read and write App.tsx content
    copy_template_file("src/app_tsx.txt", &react_app_path, "src/App.tsx");

    // Read and write index.html template content
    copy_template_file(
        "src/index_html_template.txt",
        &react_app_path,
        "public/index.html",
    );

    // Read and write index.tsx template content
    copy_template_file(
        "src/index_tsx_template.txt",
        &react_app_path,
        "src/index.tsx",
    );

    // Read and write manifest.json template content
    copy_template_file(
        "src/manifest_json_template.txt",
        &react_app_path,
        "public/manifest.json",
    );

    // Read and write tsconfig.json template content
    copy_template_file(
        "src/tsconfig_json_template.txt",
        &react_app_path,
        "tsconfig.json",
    );

  // Generate package-lock.json
    fs::write(format!("{}/package-lock.json", &react_app_path), "")
        .expect("Failed to create package-lock.json");

    // Read and write package-lock.txt content
    copy_template_file(
        "src/package-lock.txt",
        &react_app_path,
        "package-lock.json",
    );

    // Read and write .gitignore template content
    copy_template_file(
        "src/gitignore_template.txt",
        &react_app_path,
        ".gitignore",
    );

    // Read and write package.json template content
    copy_template_file(
        "src/package_json_template.txt",
        &react_app_path,
        "package.json",
    );

    // Read and write webpack.config.js template content
    copy_template_file(
        "src/webpack_config_template.txt",
        &react_app_path,
        "webpack.config.js",
    );

    println!("Project setup completed successfully!");
}

// Function to create directories based on the provided base path and directory names
fn create_directories(base_path: &str, directories: &[&str]) {
    for dir in directories {
        fs::create_dir_all(format!("{}/{}", base_path, dir))
            .expect(&format!("Failed to create {} directory", dir));
    }
}

// Function to read a template file and write its content to another file
fn copy_template_file(template_file: &str, base_path: &str, target_file: &str) {
    let content =
        fs::read_to_string(template_file).expect(&format!("Failed to read {}", template_file));
    fs::write(format!("{}/{}", base_path, target_file), content)
        .expect(&format!("Failed to add {}", target_file));
}
