import { State } from "../../../state";

import { deleteById } from "@/utils/state";

export const deleteSymbol = (set: any) => (id: string) => {
  set((state: State) => {
    return { symbols: deleteById(state.symbols, id) };
  });
};
