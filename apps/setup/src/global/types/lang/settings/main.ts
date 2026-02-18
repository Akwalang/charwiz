import {
  InjectMethodEnum,
  KeyboardLayoutEnum,
  KeyboardStateCleanupEnum,
  TransformTargetEnum,
  TransformerTypeEnum,
  UserInputCleanupEnum,
} from "@/global/types/settings/enums";

export type MainSettings = {
  fields: {
    title: {
      name: string,
    },
    description: {
      name: string,
    },
    cmd: {
      name: string,
    },
    keyboardSnapshot: {
      name: string,
    }, 
    text: {
      name: string,
    },
    symbol: {
      name: string,
    },
    injectMethod: {
      name: string,
      values: Record<InjectMethodEnum, string>,
    },
    keyboardLayout: {
      name: string,
      values: Record<KeyboardLayoutEnum, string>,
    },
    keyboardLayoutBefore: {
      name: string,
      values: Record<KeyboardLayoutEnum, string>,
    },
    keyboardLayoutAfter: {
      name: string,
      values: Record<KeyboardLayoutEnum, string>,
    },
    keyboardStateCleanup: {
      name: string,
      values: Record<KeyboardStateCleanupEnum, string>,
    },
    transformTarget: {
      name: string,
      values: Record<TransformTargetEnum, string>,
    },
    transformerType: {
      name: string,
      values: Record<TransformerTypeEnum, string>,
    },
    userInputCleanup: {
      name: string,
      values: Record<UserInputCleanupEnum, string>,
    },
  },
};
