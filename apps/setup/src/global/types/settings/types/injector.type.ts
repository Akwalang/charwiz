import { UserInputCleanupEnum } from '../enums/user-input-cleanup.enum';
import { KeyboardStateCleanupEnum } from '../enums/keyboard-state-cleanup.enum';
import { KeyboardLayoutEnum } from '../enums/keyboard-layout.enum';
import { TransformTargetEnum } from '../enums/transform-target.enum';
import { InjectMethodEnum } from '../enums/inject-method.enum';

export type Injector = {
  userInputCleanup: UserInputCleanupEnum,
  keyboardStateCleanup: KeyboardStateCleanupEnum,
  layoutBefore: KeyboardLayoutEnum | string,
  layoutAfter: KeyboardLayoutEnum | string,
  target: TransformTargetEnum,
  method: InjectMethodEnum,
};
