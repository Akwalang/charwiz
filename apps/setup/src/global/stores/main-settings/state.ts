import { CommandsState, type CommandItem } from "./slices/commands/state";
import { HotkeysState, type HotkeyItem } from "./slices/hotkeys/state";
import { SwitchesState, type SwitchItem } from "./slices/switches/state";
import { SymbolsState, type SymbolItem } from "./slices/symbols/state";

export { CommandItem as CommandItem };
export { HotkeyItem as HotkeyItem };
export { SwitchItem as SwitchItem };
export { SymbolItem as SymbolItem };

export type State =
  & CommandsState
  & HotkeysState
  & SwitchesState
  & SymbolsState;

export const State = (): State => {
  return {
    ...CommandsState(),
    ...HotkeysState(),
    ...SwitchesState(),
    ...SymbolsState(),
  } as const;
};
