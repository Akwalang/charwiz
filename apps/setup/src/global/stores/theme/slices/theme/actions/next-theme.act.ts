import { ThemeName } from '@/global/data/themes';

import { ThemeState } from '../state';

const list = Object.values(ThemeName);

export const nextTheme = (set: any) => () => set((state: ThemeState) => ({
  theme: list[(list.indexOf(state.theme) + 1) % list.length],
}));
