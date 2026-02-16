import { idxGuard, mutateListItem } from "@/utils/state";

import { State } from "../../../state";

export const commitCommand = (set: any) => (id: string) => {
  set((state: State) => {
    const idx = idxGuard('Command', state.commands, id);

    const commands = mutateListItem(state.commands, idx, (item) => ({
      isEditing: false,
      current: item.editing!,
      editing: null,
    }));

    return { commands };
  });
};
