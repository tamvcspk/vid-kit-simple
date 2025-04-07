import { useState } from 'react';
import { TabView, TabPanel } from 'primereact/tabview';
import ConvertView from './components/ConvertView';
import SplitView from './components/SplitView';
import EditView from './components/EditView';
import SanitizeView from './components/SanitizeView';
import { AppHeader } from './components/AppHeader';
import { AppFooter } from './components/AppFooter/AppFooter';
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
          <TabPanel header="Split" leftIcon="pi pi-minus">
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

      <AppFooter />
    </div>
  );
}

export default App;
