import { KeyEnum, type KeyModifierType } from "../enums/key.enum";

export type KeyboardSnapshot = {
  key: KeyEnum | null,
  modifiers: KeyModifierType[],
};
