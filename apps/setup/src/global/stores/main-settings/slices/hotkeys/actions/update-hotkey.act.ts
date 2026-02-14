import { idxGuard, mutateListItem } from "@/utils/state";
import { setter } from "@/utils/object";

import { AllPaths, PathValue } from "@/utils/types";

import { HotkeyItem } from "../state";
import { State } from "../../../state";

export const updateHotkey = (set: any) => <
  P extends AllPaths<HotkeyItem["current"]>,
  V extends PathValue<HotkeyItem["current"], P>,
>(id: string, path: P, value: V) => {
  set((state: State) => {
    const idx = idxGuard('Hotkey', state.hotkeys, id);

    const hotkeys = mutateListItem(state.hotkeys, idx, () => {
      const editing = {} as HotkeyItem["current"];

      setter(editing, path, value)

      return { editing };
    });

    return { hotkeys };
  });
};
