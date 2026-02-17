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

  return (
    <>
      <Separator />
      <InputField entity={entity} updateEntity={updateEntity} path="title" field="title" />
      <Separator />
      <InputField entity={entity} updateEntity={updateEntity} path="description" field="description" />
      <Separator />
      <InputField entity={entity} updateEntity={updateEntity} path="settings.text" field="text" />
      <Separator />
      <SelectField entity={entity} updateEntity={updateEntity} path="settings.injector.layoutBefore" field="keyboardLayoutBefore" options={Object.values(KeyboardLayoutEnum) as any} />
      <Separator />
      <SelectField entity={entity} updateEntity={updateEntity} path="settings.injector.layoutAfter" field="keyboardLayoutAfter" options={Object.values(KeyboardLayoutEnum) as any} />
    </>
  );
};
