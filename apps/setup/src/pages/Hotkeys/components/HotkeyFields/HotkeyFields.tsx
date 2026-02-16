import { Field } from "../Form/Field";
import { Separator } from "../Form/Separator";

import { InjectMethodEnum, TransformTargetEnum } from "@/global/types/settings/enums";

interface HotkeyFieldsProps {
  id: string;
}

export const HotkeyFields: React.FC<HotkeyFieldsProps> = ({ id }) => {
  return (
    <>
      <Separator />
      <Field id={id} path="settings.injector.target" field="transformTarget" options={Object.values(TransformTargetEnum) as any} />
      <Separator />
      <Field id={id} path="settings.injector.method" field="injectMethod" options={Object.values(InjectMethodEnum) as any} />
    </>
  );
};
