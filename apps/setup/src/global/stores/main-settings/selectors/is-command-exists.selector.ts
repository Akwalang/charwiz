import { State } from '../state';

export const isCommandExists = (id: string) => (state: State): boolean => {
  return !!state.commands.find((item) => item.id === id);
};
