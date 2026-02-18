import { SymbolItem } from '../state';
import { State } from '../../../state';

export const getSymbols = (state: State): SymbolItem[] => {
  return state.symbols;
};
