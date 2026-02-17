import { SwitchItem } from '../state';
import { State } from '../../../state';

export const getSwitch = (id: string) => (state: State): SwitchItem | null => {
  return state.switches.find((item) => item.id === id) || null;
};
