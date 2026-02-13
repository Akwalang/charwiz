import { ThemeState } from "./slices/theme/state";

export type State =
  & ThemeState;

export const State = (): State => ({
  ...ThemeState(),
}) as const;
