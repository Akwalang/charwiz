import { idxGuard, mutateListItem } from "@/utils/state";

import { State } from "../../../state";

export const rollbackHotkey = (set: any) => (id: string) => {
  set((state: State) => {
    const idx = idxGuard('Hotkey', state.hotkeys, id);

    const hotkeys = mutateListItem(state.hotkeys, idx, () => ({
      isEditing: false,
      editing: null,
    }));

    return { hotkeys };
  });
};
