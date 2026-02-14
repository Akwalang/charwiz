import { idxGuard, mutateListItem } from "@/utils/state";

import { State } from "../../../state";

export const commitHotkey = (set: any) => (id: string) => {
  set((state: State) => {
    const idx = idxGuard('Hotkey', state.hotkeys, id);

    const hotkeys = mutateListItem(state.hotkeys, idx, (item) => ({
      isEditing: false,
      current: item.editing!,
      editing: null,
    }));

    return { hotkeys };
  });
};
