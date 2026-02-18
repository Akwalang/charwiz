import { Separator, InputField } from "@/views/components";

import { useMainSettingsStore } from "@/global/stores/main-settings";
import { getSymbol } from "@/global/stores/main-settings/selectors";

interface SymbolFieldsProps {
  id: string;
}

export const SymbolFields: React.FC<SymbolFieldsProps> = ({ id }) => {
  const entity = useMainSettingsStore(getSymbol(id))!;

  const updateEntity = useMainSettingsStore((state) => state.updateSymbol);

  return (
    <>
      <Separator />
      <InputField entity={entity} updateEntity={updateEntity} path="settings.transformer.value" field="symbol" />
    </>
  );
};
