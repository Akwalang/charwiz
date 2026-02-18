import { KeyboardSnapshot } from "../../types/keyboard-snapshot.type";

import { CommonValues } from "./common-values.type";

export type Symbol = CommonValues & {
  keys: KeyboardSnapshot,
};
