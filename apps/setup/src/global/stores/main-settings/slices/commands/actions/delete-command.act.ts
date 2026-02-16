import { State } from "../../../state";

import { deleteById } from "@/utils/state";

export const deleteCommand = (set: any) => (id: string) => {
  set((state: State) => {
    return { commands: deleteById(state.commands, id) };
  });
};
