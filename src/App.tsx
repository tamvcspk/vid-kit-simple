import { useState } from 'react';
import { TabView, TabPanel } from 'primereact/tabview';
import { useTheme } from './hooks/useTheme';

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

function App() {
  const [activeIndex, setActiveIndex] = useState(0);
  const { isDark, toggleDarkMode, changeThemeType, } = useTheme();

  return (
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

      <Footer />
    </div>
  );
}

export default App;
