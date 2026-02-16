import { Separator, InputField, SelectField } from "@/views/components";

import { useMainSettingsStore } from "@/global/stores/main-settings";
import { getHotkey } from "@/global/stores/main-settings/selectors";

import {
  InjectMethodEnum,
  TransformTargetEnum,
  KeyboardLayoutEnum,
  UserInputCleanupEnum,
} from "@/global/types/settings/enums";

interface HotkeyFieldsProps {
  id: string;
}

export const HotkeyFields: React.FC<HotkeyFieldsProps> = ({ id }) => {
  const entity = useMainSettingsStore(getHotkey(id))!;

  const updateEntity = useMainSettingsStore((state) => state.updateHotkey);

  return (
    <>
      <Separator />
      <InputField entity={entity} updateEntity={updateEntity} path="title" field="title" />
      <Separator />
      <InputField entity={entity} updateEntity={updateEntity} path="description" field="description" />
      <Separator />
      <SelectField entity={entity} updateEntity={updateEntity} path="settings.injector.target" field="transformTarget" options={Object.values(TransformTargetEnum) as any} />
      <Separator />
      <SelectField entity={entity} updateEntity={updateEntity} path="settings.injector.method" field="injectMethod" options={Object.values(InjectMethodEnum) as any} />
      <Separator />
      <SelectField entity={entity} updateEntity={updateEntity} path="settings.injector.layoutBefore" field="keyboardLayoutBefore" options={Object.values(KeyboardLayoutEnum) as any} />
      <Separator />
      <SelectField entity={entity} updateEntity={updateEntity} path="settings.injector.layoutAfter" field="keyboardLayoutAfter" options={Object.values(KeyboardLayoutEnum) as any} />
      <Separator />
      <SelectField entity={entity} updateEntity={updateEntity} path="settings.injector.userInputCleanup" field="userInputCleanup" options={Object.values(UserInputCleanupEnum) as any} />
    </>
  );
};
