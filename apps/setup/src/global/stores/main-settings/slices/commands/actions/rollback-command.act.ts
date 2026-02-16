import { idxGuard, mutateListItem } from "@/utils/state";

import { State } from "../../../state";

export const rollbackCommand = (set: any) => (id: string) => {
  set((state: State) => {
    const idx = idxGuard('Command', state.commands, id);

    const commands = mutateListItem(state.commands, idx, () => ({
      isEditing: false,
      editing: null,
    }));

    return { commands };
  });
};
