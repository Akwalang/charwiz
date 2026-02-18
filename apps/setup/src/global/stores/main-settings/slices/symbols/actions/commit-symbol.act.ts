import { idxGuard, mutateListItem } from "@/utils/state";

import { State } from "../../../state";

export const commitSymbol = (set: any) => (id: string) => {
  set((state: State) => {
    const idx = idxGuard('Symbol', state.symbols, id);

    const symbols = mutateListItem(state.symbols, idx, (item) => ({
      isEditing: false,
      current: item.editing!,
      editing: null,
    }));

    return { symbols };
  });
};
