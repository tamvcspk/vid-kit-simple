export const THEME_TYPES = {
  LARA_INDIGO: 'Lara Indigo',
  LARA_BLUE: 'Lara Blue',
  LARA_PURPLE: 'Lara Purple',
  LARA_TEAL: 'Lara Teal',
  LARA_GREEN: 'Lara Green',
  LARA_CYAN: 'Lara Cyan',
  LARA_PINK: 'Lara Pink',
  LARA_AMBER: 'Lara Amber',
  MDC_INDIGO: 'MDC Indigo',
  MDC_DEEPPURPLE: 'MDC Deep Purple',
  MD_INDIGO: 'MD Indigo',
  MD_DEEPPURPLE: 'MD Deep Purple',
  BOOTSTRAP4_BLUE: 'Bootstrap 4 Blue',
  BOOTSTRAP4_PURPLE: 'Bootstrap 4 Purple',
  SOHO: 'Soho',
  VIVA: 'Viva',
} as const;

export const DARK_THEMES = {
  LARA_INDIGO: 'lara-dark-indigo',
  LARA_BLUE: 'lara-dark-blue',
  LARA_PURPLE: 'lara-dark-purple',
  LARA_TEAL: 'lara-dark-teal',
  LARA_GREEN: 'lara-dark-green',
  LARA_CYAN: 'lara-dark-cyan',
  LARA_PINK: 'lara-dark-pink',
  LARA_AMBER: 'lara-dark-amber',
  MDC_INDIGO: 'mdc-dark-indigo',
  MDC_DEEPPURPLE: 'mdc-dark-deeppurple',
  MD_INDIGO: 'md-dark-indigo',
  MD_DEEPPURPLE: 'md-dark-deeppurple',
  BOOTSTRAP4_BLUE: 'bootstrap4-dark-blue',
  BOOTSTRAP4_PURPLE: 'bootstrap4-dark-purple',
  SOHO: 'soho-dark',
  VIVA: 'viva-dark',
} as const;

export const LIGHT_THEMES = {
  LARA_INDIGO: 'lara-light-indigo',
  LARA_BLUE: 'lara-light-blue',
  LARA_PURPLE: 'lara-light-purple',
  LARA_TEAL: 'lara-light-teal',
  LARA_GREEN: 'lara-light-green',
  LARA_CYAN: 'lara-light-cyan',
  LARA_PINK: 'lara-light-pink',
  LARA_AMBER: 'lara-light-amber',
  MDC_INDIGO: 'mdc-light-indigo',
  MDC_DEEPPURPLE: 'mdc-light-deeppurple',
  MD_INDIGO: 'md-light-indigo',
  MD_DEEPPURPLE: 'md-light-deeppurple',
  BOOTSTRAP4_BLUE: 'bootstrap4-light-blue',
  BOOTSTRAP4_PURPLE: 'bootstrap4-light-purple',
  SOHO: 'soho-light',
  VIVA: 'viva-light',
} as const;

export const THEME_LINK_ID = 'theme-link';

export type ThemeType = keyof typeof THEME_TYPES; 