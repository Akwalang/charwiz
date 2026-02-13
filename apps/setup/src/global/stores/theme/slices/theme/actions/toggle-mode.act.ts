import { ThemeMode } from '@/global/data/themes';

import { ThemeState } from '../state';

export const toggleMode = (set: any) => () => {
  set((state: ThemeState) => ({
    mode: {
      [ThemeMode.Light]: ThemeMode.Dark,
      [ThemeMode.Dark]: ThemeMode.Light,
    }[state.mode],
  }));
};
