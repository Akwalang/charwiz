import { SymbolItem } from '../state';
import { State } from '../../../state';

export const getSymbol = (id: string) => (state: State): SymbolItem | null => {
  return state.symbols.find((item) => item.id === id) || null;
};
