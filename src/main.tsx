import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import { PrimeReactProvider } from 'primereact/api';
import { StateProviders } from './context/StateProviders';

// Initialize app
document.addEventListener('DOMContentLoaded', () => {
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
