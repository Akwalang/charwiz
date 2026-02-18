import { idxGuard, mutateListItem } from "@/utils/state";

import { State } from "../../../state";

export const rollbackSymbol = (set: any) => (id: string) => {
  set((state: State) => {
    const idx = idxGuard('Symbol', state.symbols, id);

    const symbols = mutateListItem(state.symbols, idx, () => ({
      isEditing: false,
      editing: null,
    }));

    return { symbols };
  });
};
