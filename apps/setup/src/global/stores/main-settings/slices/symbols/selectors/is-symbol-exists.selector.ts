import { State } from '../../../state';

export const isSymbolExists = (id: string) => (state: State): boolean => {
  return !!state.symbols.find((item) => item.id === id);
};
