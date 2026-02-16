import { HotkeysState, type HotkeyItem } from "./slices/hotkeys/state";

export { HotkeyItem as HotkeyItem };

export type State =
  & HotkeysState;

export const State = (): State => {
  return {
    ...HotkeysState(),
  } as const;
};
