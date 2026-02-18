import { idxGuard, mutateListItem } from "@/utils/state";

import { State } from "../../../state";

export const editSymbol = (set: any) => (id: string) => {
  set((state: State) => {
    const idx = idxGuard('Symbol', state.symbols, id);

    const symbols = mutateListItem(state.symbols, idx, (item) => ({
      isEditing: true,
      editing: item.current,
    }));

    return { symbols };
  });
};
