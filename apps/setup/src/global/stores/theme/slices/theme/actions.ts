import { nextTheme } from './actions/next-theme.act';
import { toggleMode } from './actions/toggle-mode.act';
import { setTheme } from './actions/set-theme.act';

export const ThemeActions = (set: any) => ({
  toggleMode: toggleMode(set),
  nextTheme: nextTheme(set),
  setTheme: setTheme(set),
}) as const;

export type ThemeActions = ReturnType<typeof ThemeActions>;
