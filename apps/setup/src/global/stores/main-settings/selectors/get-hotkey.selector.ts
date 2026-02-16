import { HotkeyItem } from '../slices/hotkeys/state';
import { State } from '../state';

export const getHotkey = (id: string) => (state: State): HotkeyItem | null => {
  return state.hotkeys.find((item) => item.id === id) || null;
};
