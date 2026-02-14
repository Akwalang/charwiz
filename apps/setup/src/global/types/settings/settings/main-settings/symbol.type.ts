import { KeyboardSnapshot } from "../../types/keyboard-snapshot.typs";

import { CommonValues } from "./common-values.type";

export type Symbol = CommonValues & {
  keys: KeyboardSnapshot,
};
