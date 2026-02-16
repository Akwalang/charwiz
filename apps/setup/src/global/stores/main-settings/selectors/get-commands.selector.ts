import { CommandItem } from '../slices/commands/state';
import { State } from '../state';

export const getCommands = (state: State): CommandItem[] => {
  return state.commands;
};
