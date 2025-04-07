import { useState, useEffect, useContext } from 'react';
import { PrimeReactContext } from 'primereact/api';
import { THEME_TYPES, DARK_THEMES, LIGHT_THEMES, THEME_LINK_ID, ThemeType } from '../constants/theme';

export const useTheme = () => {
  const [isDark, setIsDark] = useState(true);
  const [themeType, setThemeType] = useState<ThemeType>('LARA_INDIGO');
  const [currentTheme, setCurrentTheme] = useState<string>(THEME_TYPES.LARA_INDIGO);
  const { changeTheme } = useContext(PrimeReactContext);

  const toggleDarkMode = () => {
    const newIsDark = !isDark;
    setIsDark(newIsDark);
    localStorage.setItem('isDarkTheme', newIsDark.toString());
    applyTheme(themeType, newIsDark);
  };

  const changeThemeType = (type: ThemeType) => {
    setThemeType(type);
    localStorage.setItem('themeType', type);
    applyTheme(type, isDark);
  };

  const applyTheme = (type: ThemeType, dark: boolean) => {
    const themeName: string = dark 
      ? DARK_THEMES[type] 
      : LIGHT_THEMES[type];
    
    if (changeTheme) {
      changeTheme(currentTheme, themeName, THEME_LINK_ID, () => {
        setCurrentTheme(themeName);
        updateThemeLink(themeName);
      });
    }
  };

  const updateThemeLink = (newTheme: string) => {
    const linkElement = document.getElementById(THEME_LINK_ID);
    if (!linkElement) return;

    const currentHref = linkElement.getAttribute('href');
    if (!currentHref) return;

    const themeTokens = currentHref.split('/');
    const currentThemeName = themeTokens[themeTokens.length - 2];

    if (currentThemeName === newTheme) return;

    const newThemeUrl = currentHref.replace(currentThemeName, newTheme);
    const newLinkElement = linkElement.cloneNode(true) as HTMLLinkElement;

    newLinkElement.id = `${THEME_LINK_ID}-temp`;
    newLinkElement.href = newThemeUrl;

    newLinkElement.onload = () => {
      linkElement.remove();
      newLinkElement.id = THEME_LINK_ID;
    };

    linkElement.parentNode?.insertBefore(newLinkElement, linkElement.nextSibling);
  };

  useEffect(() => {
    const savedIsDark = localStorage.getItem('isDarkTheme');
    const savedThemeType = localStorage.getItem('themeType') as ThemeType;

    const isDarkTheme = savedIsDark === 'true';
    const themeTypeValue = savedThemeType || 'LARA_INDIGO';

    setIsDark(isDarkTheme);
    setThemeType(themeTypeValue);
    applyTheme(themeTypeValue, isDarkTheme);
  }, []);

  return {
    isDark,
    themeType,
    currentTheme,
    toggleDarkMode,
    changeThemeType,
  };
}; 