import { useState, useRef, useEffect } from 'react';
import { TabView, TabPanel } from 'primereact/tabview';
import { Toast } from 'primereact/toast';
import { useTheme } from './hooks/useTheme';
import { useNotifications } from './hooks';

// PrimeReact CSS
import 'primereact/resources/primereact.min.css';
import 'primeicons/primeicons.css';
import './App.scss';
import { Header } from './components/layout/Header';
import { ConvertView } from './features/convert';
import { SplitView } from './features/split';
import { EditView } from './features/edit';
import { SanitizeView } from './features/sanitize';

import { Footer } from './components/layout/Footer';
import { ErrorBoundary } from './components/common';

function App() {
  const [activeIndex, setActiveIndex] = useState(0);
  const { isDark, toggleDarkMode, changeThemeType, } = useTheme();
  const { notifications } = useNotifications();
  const toast = useRef<Toast>(null);

  // Show toast notifications when new notifications arrive
  useEffect(() => {
    if (notifications.length > 0) {
      const latestNotification = notifications[0];
      if (!latestNotification.read) {
        toast.current?.show({
          severity: latestNotification.type,
          summary: latestNotification.source || 'Notification',
          detail: latestNotification.message,
          life: 5000
        });
      }
    }
  }, [notifications]);

  return (
    <ErrorBoundary>
      <div className="app-container">
        <Header
          isDark={isDark}
          onToggleDarkMode={toggleDarkMode}
          onChangeThemeType={changeThemeType}
        />

        <main className="app-main">
          <TabView
            activeIndex={activeIndex}
            onTabChange={e => setActiveIndex(e.index)}
            pt={{
              root: { className: 'tabview-custom' },
              navContainer: { className: 'tabview-nav-custom' },
              panelContainer: { className: 'tabview-content-custom' },
            }}
          >
            <TabPanel header="Convert" leftIcon="pi pi-sync">
              <ErrorBoundary>
                <ConvertView />
              </ErrorBoundary>
            </TabPanel>
            <TabPanel header="Split" leftIcon="pi pi-minus">
              <ErrorBoundary>
                <SplitView />
              </ErrorBoundary>
            </TabPanel>
            <TabPanel header="Edit" leftIcon="pi pi-pencil">
              <ErrorBoundary>
                <EditView />
              </ErrorBoundary>
            </TabPanel>
            <TabPanel header="Sanitize" leftIcon="pi pi-shield">
              <ErrorBoundary>
                <SanitizeView />
              </ErrorBoundary>
            </TabPanel>

          </TabView>
        </main>

        <Footer />
        <Toast ref={toast} position="bottom-right" />
      </div>
    </ErrorBoundary>
  );
}

export default App;
