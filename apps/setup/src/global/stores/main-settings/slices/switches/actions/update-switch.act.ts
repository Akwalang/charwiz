import { idxGuard, mutateListItem } from "@/utils/state";
import { setter } from "@/utils/object";

import { AllPaths, PathValue } from "@/utils/types";

import { SwitchItem } from "../state";
import { State } from "../../../state";

export const updateSwitch = (set: any) => <
  P extends AllPaths<SwitchItem["current"]>,
  V extends PathValue<SwitchItem["current"], P>,
>(id: string, path: P, value: V) => {
  set((state: State) => {
    const idx = idxGuard('Switch', state.switches, id);

    const switches = mutateListItem(state.switches, idx, () => {
      const editing = {} as SwitchItem["current"];

      setter(editing, path, value);

      return { editing };
    });

    return { switches };
  });
};
