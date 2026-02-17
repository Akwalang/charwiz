import { SwitchItem } from '../state';
import { State } from '../../../state';

export const getSwitches = (state: State): SwitchItem[] => {
  return state.switches;
};
