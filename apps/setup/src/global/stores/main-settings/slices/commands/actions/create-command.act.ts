import { clone } from "@/utils/object";

import { State } from "../../../state";
import { DEFAULT_COMMAND } from "../constants";

export const createCommand = (set: any) => (): string => {
  const id: string = crypto.randomUUID();

  set((state: State) => {
    return { commands: [...state.commands, { id, ...clone(DEFAULT_COMMAND) } ] };
  });

  return id;
};
