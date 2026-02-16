import { MainSettings } from "@/global/types/lang/settings/main";

import {
  InjectMethodEnum,
  KeyboardLayoutEnum,
  KeyboardStateCleanupEnum,
  TransformTargetEnum,
  TransformerTypeEnum,
  UserInputCleanupEnum,
} from "@/global/types/settings/enums";

export const main: MainSettings = {
  fields: {
    title: {
      name: "Title",
    },
    description: {
      name: "Description",
    },
    cmd: {
      name: "Text command",
    },
    keyboardSnapshot: {
      name: "Keyboard keys",
    }, 
    switch: {
      name: "Text to switch from",
    },
    symbol: {
      name: "Insert symbol",
    },
    injectMethod: {
      name: "Injection method",
      values: {
        [InjectMethodEnum.Emulate]: "Rerun user actions",
        [InjectMethodEnum.Paste]: "Paste",
        [InjectMethodEnum.TypeAndPaste]: "Type and paste missing symbols",
        [InjectMethodEnum.TypeAndSkip]: "Type and skip missing symbols",
      },
    },
    keyboardLayoutBefore: {
      name: "Keyboard layout before execute",
      values: {
        [KeyboardLayoutEnum.Previous]: "Previous layout",
        [KeyboardLayoutEnum.Current]: "Current layout",
        [KeyboardLayoutEnum.Next]: "Next layout",
      },
    },
    keyboardLayoutAfter: {
      name: "Keyboard layout after execute",
      values: {
        [KeyboardLayoutEnum.Previous]: "Current layout",
        [KeyboardLayoutEnum.Current]: "Current layout",
        [KeyboardLayoutEnum.Next]: "Next layout",
      },
    },
    keyboardStateCleanup: {
      name: "Inner state cleanup",
      values: {
        [KeyboardStateCleanupEnum.None]: "Not drop",
        [KeyboardStateCleanupEnum.Drop]: "Drop",
      },
    },
    transformTarget: {
      name: "Target to transform",
      values: {
        [TransformTargetEnum.All]: "Whole document",
        [TransformTargetEnum.Clipboard]: "Clipboard",
        [TransformTargetEnum.Command]: "Command",
        [TransformTargetEnum.Events]: "User event",
        [TransformTargetEnum.Input]: "User input",
        [TransformTargetEnum.Line]: "Document line",
        [TransformTargetEnum.None]: "Nothing",
        [TransformTargetEnum.Selection]: "Selection",
        [TransformTargetEnum.Word]: "Word",
      },
    },
    transformerType: {
      name: "Transformer",
      values: {
        [TransformerTypeEnum.Native]: "Built in function",
        [TransformerTypeEnum.Plugin]: "Plugin scripts",
        [TransformerTypeEnum.Static]: "Static value",
      },
    },
    userInputCleanup: {
      name: "User input cleanup",
      values: {
        [UserInputCleanupEnum.None]: "Not clean",
        [UserInputCleanupEnum.Backspace]: "Cleanup using backspace key",
      },
    },
  },
};
