import { clone } from "@/utils/object";

import { State } from "../../../state";
import { DEFAULT_SYMBOL } from "../constants";

export const createSymbol = (set: any) => (): string => {
  const id: string = crypto.randomUUID();

  set((state: State) => {
    return { symbols: [...state.symbols, { id, ...clone(DEFAULT_SYMBOL) } ] };
  });

  return id;
};
