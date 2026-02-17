import { idxGuard, mutateListItem } from "@/utils/state";

import { State } from "../../../state";

export const rollbackSwitch = (set: any) => (id: string) => {
  set((state: State) => {
    const idx = idxGuard('Switch', state.switches, id);

    const switches = mutateListItem(state.switches, idx, () => ({
      isEditing: false,
      editing: null,
    }));

    return { switches };
  });
};
