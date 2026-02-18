export enum KeyEnum {
  Alt = "Alt",
  AltGr = "AltGr",
  ControlLeft = "ControlLeft",
  ControlRight = "ControlRight",
  ShiftLeft = "ShiftLeft",
  ShiftRight = "ShiftRight",
  MetaLeft = "MetaLeft",
  MetaRight = "MetaRight",
}

export type KeyModifierType =
  | KeyEnum.Alt
  | KeyEnum.AltGr
  | KeyEnum.ControlLeft
  | KeyEnum.ControlRight
  | KeyEnum.ShiftLeft
  | KeyEnum.ShiftRight
  | KeyEnum.MetaLeft
  | KeyEnum.MetaRight;
