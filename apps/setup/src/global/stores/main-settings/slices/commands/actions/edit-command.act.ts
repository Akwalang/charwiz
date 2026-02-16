import { idxGuard, mutateListItem } from "@/utils/state";

import { State } from "../../../state";

export const editCommand = (set: any) => (id: string) => {
  set((state: State) => {
    const idx = idxGuard('Command', state.commands, id);

    const commands = mutateListItem(state.commands, idx, (item) => ({
      isEditing: true,
      editing: item.current,
    }));

    return { commands };
  });
};
