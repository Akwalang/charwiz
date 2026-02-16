import { State } from '../state';

export const isHotkeyExists = (id: string) => (state: State): boolean => {
  return !!state.hotkeys.find((item) => item.id === id);
};
