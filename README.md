# Rust2ReactCLI - 🦀 Fast and Efficient React App Setup

### Implements the latest **React 18.2** & **TypeScript**

Rust2ReactCLI is a command-line tool designed to streamline the process of creating a responsive React app by automating various setup tasks. With Rust2ReactCLI, you can quickly set up the necessary project structure and files, saving you time and ensuring a consistent development environment across various devices.

<details>
  <summary>🌲 Project Structure Tree</summary>
<pre>
project_name/
├── public/
│ ├── index.html
│ ├── manifest.json
│ └── service-worker.js
├── src/
│ ├── Components/
│ │ ├── Home/
│ │ │ ├── Home.tsx
│ │ │ └── Home.scss
│ │ │ └── Home.test.tsx
│ │ ├── PlaceholderComponent1/
│ │ │ ├── PlaceholderComponent1.tsx
│ │ │ └── PlaceholderComponent1.scss
│ │ │ └── PlaceholderComponent1.test.tsx
│ │ └── PlaceholderComponent2/
│ │ ├── PlaceholderComponent2.tsx
│ │ └── PlaceholderComponent2.scss
│ │ └── PlaceholderComponent2.test.tsx
│ ├── Middleware/
│ │ └── middleware.tsx
│ ├── Models/
│ │ └── PlaceholderModel.tsx
│ ├── Security/
│ │ └── SetupSecurity.tsx
│ ├── Service/
│ │ └── BackendService.ts
│ ├── Styles/
│ │ ├── main.scss
│ │ └── theme.js
│ ├── Utils/
│ │ └── utils.tsx
│ ├── App.tsx
│ └── index.tsx
├── .env.development
├── .env.production
├── .gitignore
├── package.json
├── tsconfig.json
└── webpack.config.js

</pre>
</details>

## 🚀 Features

**Project Features:**

1. **Robust Project Structure:** Establish a Component-Based Architecture, ensuring modularity, scalability, and clear separation of concerns within the React application.

2. **Efficient File Templates:** Integrate fundamental files and boilerplate code, covering components, services, models, environment configurations, and a service worker for accelerated development.

3. **TypeScript-Powered React App:** Generate React applications with TypeScript, providing strong static typing for an improved developer experience.

4. **Material-UI Integration & Responsive Design:** Enhance visual appeal and user experience by seamlessly incorporating Material-UI components with responsive design for various devices.

5. **SPA with Browser Router:** Create a Single Page Application (SPA) with efficient client-side routing for smooth transitions between views, ensuring uninterrupted user navigation.

6. **Progressive Web App (PWA):** Elevate your app to PWA standards with a foundational service worker. Enable offline access, background updates and optimize caching strategies for static and dynamic content, ensuring improved offline access and responsiveness, even in challenging network conditions.

7. **Jest Testing:** Ensure application robustness with efficient and reliable Jest testing for React components.

8. **Environment Configuration:** Manage environment-specific variables through dedicated files (.env.development and .env.production) for streamlined development and production environments.

9. **React Hooks:** Utilize React Hooks for state management, side effects, and lifecycle events, promoting best practices for managing component lifecycles.

10. **Security Measures:** Implement security measures in to validate API calls, prevent SQL injection, handle Cross-Site Scripting (XSS), and manage JSON Web Tokens (JWT) for robust protection against common web vulnerabilities.

11. **Utility Functions:** Access utility functions in utils.tsx for tasks such as debouncing, whitespace checking, email format validation, currency formatting, query string parsing, and date formatting, simplifying and enhancing the development process.

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
    npm i -g yarn
    yarn install
    ```

    This command installs the necessary Node.js dependencies for your React app.

7. **Run Your React App:**

    ```bash
    yarn start
    ```

    This command starts your React app, and you can view it by navigating to `http://localhost:3000` in your web browser.

## 🚧 Additional Resources

- [Node.js Installation](https://nodejs.org/dist/latest-v16.x/): Download and install Node.js version >16.20.2 manually if needed.

## 🙏 Acknowledgments

Rust2ReactCLI is powered by the [dialoguer](https://crates.io/crates/dialoguer) crate for interactive command-line prompts.

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
