import { idxGuard, mutateListItem } from "@/utils/state";

import { State } from "../../../state";

export const commitSwitch = (set: any) => (id: string) => {
  set((state: State) => {
    const idx = idxGuard('Switch', state.switches, id);

    const switches = mutateListItem(state.switches, idx, (item) => ({
      isEditing: false,
      current: item.editing!,
      editing: null,
    }));

    return { switches };
  });
};
