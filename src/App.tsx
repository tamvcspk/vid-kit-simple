import { useState } from 'react';
import { TabView, TabPanel } from 'primereact/tabview';
import ConvertView from './components/ConvertView';
import SplitView from './components/SplitView';
import EditView from './components/EditView';
import SanitizeView from './components/SanitizeView';
import { AppHeader } from './components/AppHeader';
import { useTheme } from './hooks/useTheme';

// PrimeReact CSS
import 'primereact/resources/primereact.min.css';
import 'primeicons/primeicons.css';
import './App.scss';

function App() {
  const [activeIndex, setActiveIndex] = useState(0);
  const { isDark, toggleDarkMode, changeThemeType, } = useTheme();

  return (
    <div className="app-container">
      <AppHeader 
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
            <ConvertView />
          </TabPanel>
          <TabPanel header="Split" leftIcon="pi pi-scissors">
            <SplitView />
          </TabPanel>
          <TabPanel header="Edit" leftIcon="pi pi-pencil">
            <EditView />
          </TabPanel>
          <TabPanel header="Sanitize" leftIcon="pi pi-shield">
            <SanitizeView />
          </TabPanel>
        </TabView>
      </main>

      <footer className="app-footer">
        <div className="gpu-status">
          <i className="pi pi-desktop" />
          <span>GPU: Not Detected</span>
        </div>
      </footer>
    </div>
  );
}

export default App;
