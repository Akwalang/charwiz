import { clone } from "@/utils/object";

import { State } from "../../../state";
import { DEFAULT_SWITCH } from "../constants";

export const createSwitch = (set: any) => (): string => {
  const id: string = crypto.randomUUID();

  set((state: State) => {
    return { switches: [...state.switches, { id, ...clone(DEFAULT_SWITCH) } ] };
  });

  return id;
};
