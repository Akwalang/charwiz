import { idxGuard, mutateListItem } from "@/utils/state";
import { setter } from "@/utils/object";

import { AllPaths, PathValue } from "@/utils/types";

import { CommandItem } from "../state";
import { State } from "../../../state";

export const updateCommand = (set: any) => <
  P extends AllPaths<CommandItem["current"]>,
  V extends PathValue<CommandItem["current"], P>,
>(id: string, path: P, value: V) => {
  set((state: State) => {
    const idx = idxGuard('Command', state.commands, id);

    const commands = mutateListItem(state.commands, idx, () => {
      const editing = {} as CommandItem["current"];

      setter(editing, path, value);

      return { editing };
    });

    return { commands };
  });
};
