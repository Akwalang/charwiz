import { CommandItem } from '../slices/commands/state';
import { State } from '../state';

export const getCommand = (id: string) => (state: State): CommandItem | null => {
  return state.commands.find((item) => item.id === id) || null;
};
