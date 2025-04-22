import { useRef, useEffect } from 'react';
import { TabView, TabPanel } from 'primereact/tabview';
import { Toast } from 'primereact/toast';
import { useTheme } from './hooks/useTheme';
import { useNotifications, useAppState, useConversionState, usePreferences } from './hooks';

// PrimeReact CSS
import 'primereact/resources/primereact.min.css';
import 'primeicons/primeicons.css';
import './App.scss';
import { Header } from './components/layout/Header';
import { ConvertView } from './features/convert';
import { SplitView } from './features/split';
import { EditView } from './features/edit';
import { SanitizeView } from './features/sanitize';
import { TaskQueueView } from './features/tasks';

import { Footer } from './components/layout/Footer';
import { ErrorBoundary, AppInitializer } from './components/common';

// Helper function to convert tab name to index
function getTabIndex(tabName: string): number {
  switch (tabName) {
    case 'convert': return 0;
    case 'split': return 1;
    case 'edit': return 2;
    case 'sanitize': return 3;
    case 'tasks': return 4;
    default: return 0;
  }
}

// Helper function to convert index to tab name
function getTabName(index: number): 'convert' | 'split' | 'edit' | 'sanitize' | 'tasks' {
  switch (index) {
    case 0: return 'convert';
    case 1: return 'split';
    case 2: return 'edit';
    case 3: return 'sanitize';
    case 4: return 'tasks';
    default: return 'convert';
  }
}

function App() {
  // Initialize state
  const { activeTab, setActiveTab } = useAppState();
  const { isDark, toggleDarkMode, changeThemeType } = useTheme();
  const { notifications } = useNotifications();
  const toast = useRef<Toast>(null);

  // AppInitializer will handle loading data

  // Handle tab change
  const handleTabChange = (index: number) => {
    setActiveTab(getTabName(index));
  };

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
        <AppInitializer />
        <Header
          isDark={isDark}
          onToggleDarkMode={toggleDarkMode}
          onChangeThemeType={changeThemeType}
        />

        <main className="app-main">
          <TabView
            activeIndex={getTabIndex(activeTab)}
            onTabChange={e => handleTabChange(e.index)}
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
            <TabPanel header="Tasks" leftIcon="pi pi-list">
              <ErrorBoundary>
                <TaskQueueView />
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
