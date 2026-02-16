import { CommandsActions } from "./slices/commands/actions";
import { HotkeysActions } from "./slices/hotkeys/actions";

export const Actions = (set: any) => ({
  ...CommandsActions(set),
  ...HotkeysActions(set),
});

export type Actions = ReturnType<typeof Actions>;
