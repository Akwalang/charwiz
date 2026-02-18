import {
  InjectMethodEnum,
  KeyboardLayoutEnum,
  KeyboardStateCleanupEnum,
  TransformerTypeEnum,
  TransformTargetEnum,
  UserInputCleanupEnum,
} from "@/global/types/settings/enums";

import { SymbolItem } from "./state";

export const NEW_SYMBOL_TITLE = "Symbol title";
export const NEW_SYMBOL_DESCRIPTION = "Symbol description";

export const DEFAULT_SYMBOL: Omit<SymbolItem, 'id'> = {
  isEditing: false,
  current: {
    title: NEW_SYMBOL_TITLE,
    description: NEW_SYMBOL_DESCRIPTION,
    settings: {
      keys: {
        key: null,
        modifiers: [],
      },
      transformer: {
        type: TransformerTypeEnum.Static,
        value: "",
      },
      injector: {
        target: TransformTargetEnum.None,
        keyboardStateCleanup: KeyboardStateCleanupEnum.None,
        layoutBefore: KeyboardLayoutEnum.Current,
        layoutAfter: KeyboardLayoutEnum.Current,
        method: InjectMethodEnum.Paste,
        userInputCleanup: UserInputCleanupEnum.None,
      },
    },
  },
  editing: null,
};
