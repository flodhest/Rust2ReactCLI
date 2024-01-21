// src/index.tsx
import { createRoot } from 'react-dom/client';
import React from 'react';
import { BrowserRouter as Router } from 'react-router-dom';
import App from './App';
import './styles/main.scss';

// Register the service worker for PWA
if ('serviceWorker' in navigator) {
  const serviceWorkerPath = `${process.env.PUBLIC_URL}/service-worker.js`;

  navigator.serviceWorker
    .register(serviceWorkerPath)
    .then((registration) => {
      console.log('Service Worker registered with scope:', registration.scope);
    })
    .catch((error) => {
      console.error('Error registering Service Worker:', error);
    });
}

const rootElement = document.getElementById('root');

if (rootElement) {
  const root = createRoot(rootElement);
  root.render(
    <Router>
      <React.StrictMode>
        <App />
      </React.StrictMode>
    </Router>
  );
} else {
  console.error('Root element not found.');
}
