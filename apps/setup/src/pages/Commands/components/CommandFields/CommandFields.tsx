import { Separator, InputField, SelectField } from "@/views/components";

import { getCommand } from "@/global/stores/main-settings/selectors";
import { useMainSettingsStore } from "@/global/stores/main-settings";

import {
  InjectMethodEnum,
  TransformTargetEnum,
} from "@/global/types/settings/enums";


const transformTarget = [
  TransformTargetEnum.None,
  TransformTargetEnum.Input,
] as any;

const injectMethod = [
  InjectMethodEnum.Paste,
  InjectMethodEnum.TypeAndPaste,
  InjectMethodEnum.TypeAndSkip,
] as any;

interface CommandFieldsProps {
  id: string;
}

export const CommandFields: React.FC<CommandFieldsProps> = ({ id }) => {
  const common = {
    entity: useMainSettingsStore(getCommand(id))!,
    updateEntity: useMainSettingsStore((state) => state.updateCommand),
  };

  return (
    <>
      <Separator />
      <InputField {...common} path="title" field="title" />
      <Separator />
      <InputField {...common} path="description" field="description" />
      <Separator />
      <InputField {...common} path="settings.cmd" field="cmd" />
      <Separator />
      <SelectField {...common} path="settings.injector.target" field="transformTarget" options={transformTarget} />
      <Separator />
      <SelectField {...common} path="settings.injector.method" field="injectMethod" options={injectMethod} />
    </>
  );
};
