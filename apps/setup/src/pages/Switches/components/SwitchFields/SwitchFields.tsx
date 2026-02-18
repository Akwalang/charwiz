import { useCallback } from "react";

import { Separator, InputField, SelectField } from "@/views/components";

import { useMainSettingsStore } from "@/global/stores/main-settings";
import { getSwitch } from "@/global/stores/main-settings/selectors";

import { KeyboardLayoutEnum } from "@/global/types/settings/enums";

interface SwitchFieldsProps {
  id: string;
}

export const SwitchFields: React.FC<SwitchFieldsProps> = ({ id }) => {
  const entity = useMainSettingsStore(getSwitch(id))!;

  const updateEntity = useMainSettingsStore((state) => state.updateSwitch);

  const updateKeyboardLayouts = useCallback((id: string, _path: string, value: KeyboardLayoutEnum | string) => {
    updateEntity(id, "settings.injector.layoutBefore", value);
    updateEntity(id, "settings.injector.layoutAfter", value);
  }, []);

  return (
    <>
      <Separator />
      <InputField entity={entity} updateEntity={updateEntity} path="settings.text" field="text" />
      <Separator />
      <SelectField entity={entity} updateEntity={updateKeyboardLayouts} path="settings.injector.layoutBefore" field="keyboardLayout" options={Object.values(KeyboardLayoutEnum) as any} />
    </>
  );
};
