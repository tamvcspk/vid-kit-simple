import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import { PrimeReactProvider } from 'primereact/api';
import { StateProviders } from './context/StateProviders';
import { startTaskCleanupScheduler } from './utils/task-cleanup';
import { tauriLogService } from './services/tauriLogService';

// Initialize app
document.addEventListener('DOMContentLoaded', async () => {
  // Start task cleanup scheduler
  startTaskCleanupScheduler();

  // Log application initialization
  await tauriLogService.logInfo('Application frontend initialized');
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
