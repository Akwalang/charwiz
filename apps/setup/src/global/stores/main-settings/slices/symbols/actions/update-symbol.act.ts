import { idxGuard, mutateListItem } from "@/utils/state";
import { setter } from "@/utils/object";

import { AllPaths, PathValue } from "@/utils/types";

import { SymbolItem } from "../state";
import { State } from "../../../state";

export const updateSymbol = (set: any) => <
  P extends AllPaths<SymbolItem["current"]>,
  V extends PathValue<SymbolItem["current"], P>,
>(id: string, path: P, value: V) => {
  set((state: State) => {
    const idx = idxGuard('Symbol', state.symbols, id);

    const symbols = mutateListItem(state.symbols, idx, () => {
      const editing = {} as SymbolItem["current"];

      setter(editing, path, value);

      return { editing };
    });

    return { symbols };
  });
};
