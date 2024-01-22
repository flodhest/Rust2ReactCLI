// Home.tsx
import React from 'react';
import './Home.scss'; 

const Home: React.FC = () => (
    <div className="intro">
    <div className="features">
      <h2 className="feature-heading">🚀 Features</h2>
      <ul className="feature-list">
  
       
          <strong>Project Structure:</strong> Establishes a robust and organized architecture by configuring essential directories, such as services, models, components, styles, middleware, utility, and the source code itself. The design adheres to a Component-Based Architecture, ensuring modularity, scalability, and a clear separation of concerns within the React application.

        <li><br></br>
          <strong>File Templates:</strong> Integrates fundamental files and boilerplate code into the project, covering key elements like placeholder components, backend services, models, environment configuration files, and a service worker. These templates expedite the development process and provide a solid foundation for building a feature-rich React application.
        </li><br></br>
        <li>
          <strong>TypeScript-Based React App:</strong> Generates a React app using TypeScript, providing strong static typing and an improved developer experience.
        </li><br></br>
        <li>
          <strong>Material-UI Integration:</strong> Enhances the visual appeal and user experience by seamlessly incorporating Material-UI components. This integration ensures a cohesive and visually pleasing user interface, following best practices in design.
        </li><br></br>
        <li>
          <strong>Responsive Design:</strong> Integrates media queries in the main stylesheet (`main.scss`) for optimal viewing experiences on various devices. Includes styles for mobile, tablets, and desktop, ensuring a responsive and adaptable user interface.
        </li><br></br>
        <li>
          <strong>SPA and Browser Router:</strong> Transforms your React application into a Single Page Application (SPA), providing smooth and uninterrupted user navigation. The inclusion of a Browser Router enables efficient client-side routing, enhancing the overall user experience with seamless transitions between different views.
        </li><br></br>
        <li>
          <strong>Progressive Web App (PWA):</strong> Elevates your application to Progressive Web App standards by implementing a foundational service worker. This feature enables offline access, background updates, and an enhanced user experience, aligning your React app with modern web development trends.
        </li><br></br>
        <li>
          <strong>Environment Files:</strong> Creates environment configuration files (`.env.development` and `.env.production`) to manage environment-specific variables. These files allow you to define variables like API endpoints or feature toggles for development and production environments separately.
        </li>
      </ul>
    </div>
  </div>
);

export default Home;
