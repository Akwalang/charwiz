import { CommandsActions } from "./slices/commands/actions";
import { HotkeysActions } from "./slices/hotkeys/actions";
import { SwitchesActions } from "./slices/switches/actions";

export const Actions = (set: any) => ({
  ...CommandsActions(set),
  ...HotkeysActions(set),
  ...SwitchesActions(set),
});

export type Actions = ReturnType<typeof Actions>;
