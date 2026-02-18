import { Controls } from "@/views/components";

import { SymbolFields } from "../SymbolFields/SymbolFields";

import { useMainSettingsStore } from "@/global/stores/main-settings";
import { getSymbol } from "@/global/stores/main-settings/selectors";

import { DEFAULT_SYMBOL } from "@/global/stores/main-settings/slices/symbols/constants";

interface SymbolProps {
  id: string,
}

export const Symbol: React.FC<SymbolProps> = ({ id }) => {
  const entity = useMainSettingsStore(getSymbol(id))!;

  const controlsActions = {
    editEntity: useMainSettingsStore((state) => state.editSymbol),
    saveEntity: useMainSettingsStore((state) => state.commitSymbol),
    cancelEntity: useMainSettingsStore((state) => state.rollbackSymbol),
    deleteEntity: useMainSettingsStore((state) => state.deleteSymbol),
  };

  return (
    <div className="w-full px-6 border rounded-2xl flex flex-col items-stretch">
      <div className="w-full py-4 flex items-center justify-between">
        <div className="flex grow flex-col justify-center">
          <p className="font-medium">{entity.current.title}</p>
          {entity.current.description && <p className="text-sm text-foreground/60">{entity.current.description}</p>}
        </div>
        <div className="flex gap-2">
          <Controls entity={entity} defaultValue={DEFAULT_SYMBOL} {...controlsActions} />
        </div>
      </div>
      {entity.isEditing && <SymbolFields id={id} />}
    </div>
  );
};
