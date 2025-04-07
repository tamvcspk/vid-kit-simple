import React from 'react';
import ReactDOM from 'react-dom/client';
import { invoke } from '@tauri-apps/api/core';
import App from './App';
import { PrimeReactProvider } from 'primereact/api';

// Initialize app
document.addEventListener('DOMContentLoaded', () => {
  // Check GPU availability on startup
  invoke('check_gpu_availability').catch(console.error);
});

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <PrimeReactProvider>
      <App />
    </PrimeReactProvider>
  </React.StrictMode>
);
