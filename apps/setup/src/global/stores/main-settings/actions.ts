import { HotkeysActions } from "./slices/hotkeys/actions";

export const Actions = (set: any) => ({
  ...HotkeysActions(set),
});

export type Actions = ReturnType<typeof Actions>;
