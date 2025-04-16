import { ReactNode } from 'react';
import { GlobalStateProvider } from './GlobalStateProvider';

// Component combines all providers
export function StateProviders({ children }: { children: ReactNode }) {
  return (
    <GlobalStateProvider>
      {children}
    </GlobalStateProvider>
  );
}
