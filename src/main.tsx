import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import { PrimeReactProvider } from 'primereact/api';
import { StateProviders } from './context/StateProviders';
import { startTaskCleanupScheduler } from './utils/task-cleanup';

// Initialize app
document.addEventListener('DOMContentLoaded', () => {
  // Start task cleanup scheduler
  startTaskCleanupScheduler();
});

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <PrimeReactProvider>
      <StateProviders>
        <App />
      </StateProviders>
    </PrimeReactProvider>
  </React.StrictMode>
);
