import { State } from '../../../state';

export const isSwitchExists = (id: string) => (state: State): boolean => {
  return !!state.switches.find((item) => item.id === id);
};
