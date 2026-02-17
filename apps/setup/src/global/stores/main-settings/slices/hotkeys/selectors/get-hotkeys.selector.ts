import { HotkeyItem } from '../state';
import { State } from '../../../state';

export const getHotkeys = (state: State): HotkeyItem[] => {
  return state.hotkeys;
};
