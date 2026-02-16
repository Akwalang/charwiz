import { CommandsState, type CommandItem } from "./slices/commands/state";
import { HotkeysState, type HotkeyItem } from "./slices/hotkeys/state";

export { HotkeyItem as HotkeyItem };
export { CommandItem as CommandItem };

export type State =
  & CommandsState
  & HotkeysState;

export const State = (): State => {
  return {
    ...CommandsState(),
    ...HotkeysState(),
  } as const;
};
