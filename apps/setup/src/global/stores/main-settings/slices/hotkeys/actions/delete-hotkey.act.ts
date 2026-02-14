import { State } from "../../../state";

import { deleteById } from "@/utils/state";

export const deleteHotkey = (set: any) => (id: string) => {
  set((state: State) => {
    return { hotkeys: deleteById(state.hotkeys, id) };
  });
};
