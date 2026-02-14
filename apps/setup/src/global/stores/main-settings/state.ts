import { HotkeysState } from "./slices/hotkeys/state";

export type State =
  & HotkeysState;

export const State = (): State => {
  return {
    ...HotkeysState(),
  } as const;
};
