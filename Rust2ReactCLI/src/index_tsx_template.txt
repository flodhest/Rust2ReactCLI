import { createRoot } from 'react-dom/client';
import React from 'react';
import App from './App';

// Add type declarations for global objects
declare const navigator: Navigator;
declare const window: Window;

// Register the service worker for PWA
if ('serviceWorker' in navigator) {
  const serviceWorkerPath = `${process.env.PUBLIC_URL}/service-worker.js`;

  navigator.serviceWorker
    .register(serviceWorkerPath)
    .then((registration: ServiceWorkerRegistration) => {
      console.log('Service Worker registered with scope:', registration.scope);
    })
    .catch((error: Error) => {
      console.error('Error registering Service Worker:', error);
    });
}

const rootElement = window.document.getElementById('root');

if (rootElement) {
  const root = createRoot(rootElement);
  root.render(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );
} else {
  console.error('Root element not found.');
}
