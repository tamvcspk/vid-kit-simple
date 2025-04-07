import { useRef } from 'react';
import { Button } from 'primereact/button';
import { Menu } from 'primereact/menu';
import { MenuItem } from 'primereact/menuitem';
import vidkitLogo from '../assets/vidkit-logo.svg';
import { THEME_TYPES, ThemeType } from '../constants/theme';

interface AppHeaderProps {
  isDark: boolean;
  onToggleDarkMode: () => void;
  onChangeThemeType: (type: ThemeType) => void;
}

export const AppHeader = ({ isDark, onToggleDarkMode, onChangeThemeType }: AppHeaderProps) => {
  const themeMenuRef = useRef<Menu>(null);

  const themeMenuItems: MenuItem[] = Object.keys(THEME_TYPES).map(theme => ({
    label: THEME_TYPES[theme as ThemeType],
    icon: 'pi pi-palette',
    command: () => onChangeThemeType(theme as ThemeType),
  }));

  const showThemeMenu = (event: React.MouseEvent) => {
    themeMenuRef.current?.toggle(event);
  };

  return (
    <header className="app-header">
      <img src={vidkitLogo} className="app-logo" alt="VidKit logo" />
      <h1>VidKit Simple</h1>
      <div className="header-actions">
        <Button
          className="p-button-rounded p-button-text mr-2"
          icon={isDark ? 'pi pi-sun' : 'pi pi-moon'}
          tooltip={isDark ? 'Switch to Light Mode' : 'Switch to Dark Mode'}
          tooltipOptions={{ event: 'hover', position: 'bottom' }}
          onClick={onToggleDarkMode}
        />
        <Button
          className="p-button-rounded p-button-text"
          icon="pi pi-palette"
          tooltip="Select Theme"
          tooltipOptions={{ event: 'hover', position: 'bottom' }}
          onClick={showThemeMenu}
        />
        <Menu model={themeMenuItems} popup ref={themeMenuRef} />
      </div>
    </header>
  );
}; 