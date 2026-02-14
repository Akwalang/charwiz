import { KeyboardModifiers } from "./keyboard-modifiers.type";
import { KeyEnum } from "../enums/key.enum";

export type KeyboardSnapshot = {
  key: KeyEnum | null,
  modifiers: KeyboardModifiers,
};
