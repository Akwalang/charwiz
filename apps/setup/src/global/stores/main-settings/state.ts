import { CommandsState, type CommandItem } from "./slices/commands/state";
import { HotkeysState, type HotkeyItem } from "./slices/hotkeys/state";
import { SwitchesState, type SwitchItem } from "./slices/switches/state";

export { CommandItem as CommandItem };
export { HotkeyItem as HotkeyItem };
export { SwitchItem as SwitchItem };

export type State =
  & CommandsState
  & HotkeysState
  & SwitchesState;

export const State = (): State => {
  return {
    ...CommandsState(),
    ...HotkeysState(),
    ...SwitchesState(),
  } as const;
};
