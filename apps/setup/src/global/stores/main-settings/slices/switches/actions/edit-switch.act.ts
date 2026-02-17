import { idxGuard, mutateListItem } from "@/utils/state";

import { State } from "../../../state";

export const editSwitch = (set: any) => (id: string) => {
  set((state: State) => {
    const idx = idxGuard('Switch', state.switches, id);

    const switches = mutateListItem(state.switches, idx, (item) => ({
      isEditing: true,
      editing: item.current,
    }));

    return { switches };
  });
};
