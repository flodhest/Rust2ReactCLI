# RustCLI - 🦀 Fast and Efficient React App Setup

RustCLI is a command-line tool designed to streamline the process of creating a responsive React app by automating various setup tasks. With RustCLI, you can quickly set up the necessary project structure and files, saving you time and ensuring a consistent development environment across various devices.

## 🚀 Features

- **Project Structure:** Sets up a well-organized project structure with directories for services, models, components, styles, and source code.

- **File Templates:** Adds essential files and boilerplate code, including placeholder components, backend service, models, environment files, and a service worker.

- **TypeScript-Based React App:** Generates a React app using TypeScript, providing strong static typing and improved developer experience.

- **SPA and Browser Router:** Configures your React app as a Single Page Application (SPA) with browser routing for seamless navigation.

- **Progressive Web App (PWA):** Sets up a basic service worker to enable Progressive Web App features, enhancing the user experience.

- **Responsive Design:** Integrates media queries in the main stylesheet (`main.scss`) for optimal viewing experiences on various devices. Includes styles for mobile, tablets, and desktop.

- **Environment Files:** Creates environment configuration files (`.env.development` and `.env.production`) to manage environment-specific variables. These files allow you to define variables like API endpoints or feature toggles for development and production environments separately.

## 🛠 Prerequisites

Before using RustCLI, ensure that you have Node.js version >14.x - <15 installed on your machine. If not, RustCLI will guide you through the installation process.

## 🏁 Getting Started

1. **Clone the RustCLI repository to your local machine:**

    ```bash
    git clone https://github.com/flodhest/rustcli.git
    cd rustcli
    ```

2. **Ensure Rust is Installed:**

    Before proceeding, make sure you have Rust installed on your machine. If not, you can install Rust by running:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    Follow the instructions to complete the installation.

3. **Build and Run RustCLI:**

    ```bash
    cargo run
    ```

    Follow the prompts to enter the project name and make choices regarding Node.js installation.

4. **Project Setup:**

    Once completed, your React app project will be set up with the specified structure and files.

5. **Navigate to Your React App:**

    ```bash
    cd your_project_name
    ```

6. **Install Node.js Dependencies:**

    ```bash
    npm install
    ```

    This command installs the necessary Node.js dependencies for your React app.

7. **Run Your React App:**

    ```bash
    npm start
    ```

    This command starts your React app, and you can view it by navigating to `http://localhost:3000` in your web browser.


## 📁 Project Structure

The project structure created by RustCLI includes the following directories:

- `src/Service`: Backend service-related files.
- `src/Models`: Data model definitions.
- `src/Components`: React components.
- `src/Styles`: Styling files.
- `src`: Main source code directory.

## 🎨 Customization

Feel free to modify the provided templates or add your own files to suit your project requirements. RustCLI aims to provide a solid starting point that you can build upon.

## 🚧 Additional Resources

- [Node.js Installation](https://nodejs.org/en/download/): Download and install Node.js version 14.x - < 15 manually if needed.

## 🙏 Acknowledgments

RustCLI is powered by the [dialoguer](https://crates.io/crates/dialoguer) crate for interactive command-line prompts and [open](https://crates.io/crates/open) crate for opening a URL.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
