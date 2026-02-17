import { State } from "../../../state";

import { deleteById } from "@/utils/state";

export const deleteSwitch = (set: any) => (id: string) => {
  set((state: State) => {
    return { switches: deleteById(state.switches, id) };
  });
};
