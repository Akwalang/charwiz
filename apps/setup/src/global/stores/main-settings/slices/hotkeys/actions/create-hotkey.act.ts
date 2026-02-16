import { clone } from "@/utils/object";

import { State } from "../../../state";
import { DEFAULT_HOTKEY } from "../constants";

export const createHotkey = (set: any) => (): string => {
  const id: string = crypto.randomUUID();

  set((state: State) => {
    return { hotkeys: [...state.hotkeys, { id, ...clone(DEFAULT_HOTKEY) } ] };
  });

  return id;
};
