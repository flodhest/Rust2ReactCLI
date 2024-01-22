# Rust2ReactCLI - 🦀 Fast and Efficient React App Setup

### Implements the latest **React 18.2** & **TypeScript 5.3**

Rust2ReactCLI is a command-line tool designed to streamline the process of creating a responsive React app by automating various setup tasks. With Rust2ReactCLI, you can quickly set up the necessary project structure and files, saving you time and ensuring a consistent development environment across various devices.

## 🌲 FileTree

<details>
  <summary>🌲 Project Structure Tree</summary>
arduino
Copy code
project_name/
├── public/
│   ├── index.html
│   ├── manifest.json
│   └── service-worker.js
├── src/
│   ├── Components/
│   │   ├── Home/
│   │   │   ├── Home.tsx
│   │   │   └── Home.test.tsx
│   │   ├── PlaceholderComponent1/
│   │   │   ├── PlaceholderComponent1.tsx
│   │   │   └── PlaceholderComponent1.test.tsx
│   │   └── PlaceholderComponent2/
│   │       ├── PlaceholderComponent2.tsx
│   │       └── PlaceholderComponent2.test.tsx
│   ├── Middleware/
│   │   └── middleware.tsx
│   ├── Models/
│   │   └── PlaceholderModel.tsx
│   ├── Service/
│   │   └── BackendService.ts
│   ├── Styles/
│   │   ├── main.scss
│   │   └── theme.js
│   ├── Utils/
│   │   └── utils.tsx
│   ├── App.tsx
│   └── index.tsx
├── .env.development
├── .env.production
├── .gitignore
├── package.json
├── tsconfig.json
└── webpack.config.js
</details>

## 🚀 Features

- **Project Structure:** Establishes a robust and organized architecture by configuring essential directories, such as services, models, components, styles, middleware, utility, and the source code itself. The design adheres to a Component-Based Architecture, ensuring modularity, scalability, and a clear separation of concerns within the React application.

- **File Templates:** Integrates fundamental files and boilerplate code into the project, covering key elements like placeholder components, backend services, models, environment configuration files, and a service worker. These templates expedite the development process and provide a solid foundation for building a feature-rich React application.

- **TypeScript-Based React App:** Generates a React app using TypeScript, providing strong static typing and an improved developer experience.

- **Material-UI I& Responsive Design:** Enhances the visual appeal and user experience by seamlessly incorporating Material-UI components. Includes setup for either Material-UI or custom media-queries for mobile, tablets, and desktop.

- **SPA and Browser Router:** Create a Single Page Application (SPA), providing smooth and uninterrupted user navigation. The inclusion of a Browser Router enables efficient client-side routing, enhancing the overall user experience with seamless transitions between different views.

- **Progressive Web App (PWA):** Elevates your application to Progressive Web App standards by registring a foundational service worker. This feature enables offline access, background updates, and an enhanced user experience.

- **Jest Testing:** Includes Jest for efficient and reliable unit testing of your React components, ensuring the robustness and reliability of your application.

- **Environment Files:** Creates environment configuration files (`.env.development` and `.env.production`) to manage environment-specific variables. These files allow you to define variables like API endpoints or feature toggles for development and production environments separately.


## 🛠 Prerequisites

Before using Rust2ReactCLI, ensure that you have Node.js version >=16.20.2 installed on your machine. https://nodejs.org/dist/latest-v16.x/

## 🏁 Getting Started

1. **Clone the Rust2ReactCLI repository to your local machine:**

    ```bash
    git clone https://github.com/flodhest/Rust2ReactCLI.git
    cd Rust2ReactCLI
    ```

2. **Ensure Rust is Installed:**

    Before proceeding, make sure you have Rust installed on your machine. If not, you can install Rust by running:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    Follow the instructions to complete the installation.

3. **Build and Run Rust2ReactCLI:**

    ```bash
    cargo run
    ```

    Follow the prompts to enter the project name, the project will be generated in Rust2ReactCLI/src. Run the terminal from the new projects folder. 

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

## 🚧 Additional Resources

- [Node.js Installation](https://nodejs.org/dist/latest-v16.x/): Download and install Node.js version >16.20.2 manually if needed.

## 🙏 Acknowledgments

Rust2ReactCLI is powered by the [dialoguer](https://crates.io/crates/dialoguer) crate for interactive command-line prompts.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
