import { KeyboardSnapshot } from "../../types/keyboard-snapshot.typs";

import { CommonValues } from "./common-values.type";

export type Hotkey = CommonValues & {
  keys: KeyboardSnapshot,
};
