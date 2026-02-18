import { Controls } from "@/views/components";

import { SwitchFields } from "../SwitchFields/SwitchFields";

import { useMainSettingsStore } from "@/global/stores/main-settings";
import { getSwitch } from "@/global/stores/main-settings/selectors";

import { DEFAULT_SWITCH } from "@/global/stores/main-settings/slices/switches/constants";

interface SwitchProps {
  id: string,
}

export const Switch: React.FC<SwitchProps> = ({ id }) => {
  const entity = useMainSettingsStore(getSwitch(id))!;

  const controlsActions = {
    editEntity: useMainSettingsStore((state) => state.editSwitch),
    saveEntity: useMainSettingsStore((state) => state.commitSwitch),
    cancelEntity: useMainSettingsStore((state) => state.rollbackSwitch),
    deleteEntity: useMainSettingsStore((state) => state.deleteSwitch),
  };

  return (
    <div className="w-full px-6 border rounded-2xl flex flex-col items-stretch">
      <div className="w-full py-4 flex items-center justify-between">
        <div className="flex grow flex-col justify-center">
          <div className="font-medium">From: {entity.current.settings.text || "Not set"}</div>
          <div className="font-medium">To: {entity.current.settings.text || "Not set"}</div>
        </div>
        <div className="flex gap-2">
          <Controls entity={entity} defaultValue={DEFAULT_SWITCH} {...controlsActions} />
        </div>
      </div>
      {entity.isEditing && <SwitchFields id={id} />}
    </div>
  );
};
