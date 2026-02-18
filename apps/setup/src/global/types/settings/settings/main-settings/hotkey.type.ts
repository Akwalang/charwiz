import { KeyboardSnapshot } from "../../types/keyboard-snapshot.type";

import { CommonValues } from "./common-values.type";

export type Hotkey = CommonValues & {
  keys: KeyboardSnapshot,
};
