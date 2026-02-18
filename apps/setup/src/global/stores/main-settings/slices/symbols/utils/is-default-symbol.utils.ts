import { isSuperSet } from "@/utils/object";

import { SymbolItem } from "../state";
import { DEFAULT_SYMBOL } from "../constants";

export const isDefaultSymbol = (symbol: SymbolItem): boolean => {
  return isSuperSet(symbol, DEFAULT_SYMBOL);
};
