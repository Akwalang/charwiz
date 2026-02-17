import { CommandItem } from '../state';
import { State } from '../../../state';

export const getCommands = (state: State): CommandItem[] => {
  return state.commands;
};
