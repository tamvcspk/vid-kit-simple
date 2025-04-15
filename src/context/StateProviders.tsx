import { ReactNode } from 'react';
import { GlobalStateProvider } from './GlobalStateProvider';

// Component kết hợp tất cả các provider
export function StateProviders({ children }: { children: ReactNode }) {
  return (
    <GlobalStateProvider>
      {children}
    </GlobalStateProvider>
  );
}
