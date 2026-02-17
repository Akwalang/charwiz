import { Controls } from "@/views/components";

import { HotkeyFields } from "../HotkeyFields/HotkeyFields";

import { useMainSettingsStore } from "@/global/stores/main-settings";
import { getHotkey } from "@/global/stores/main-settings/selectors";

import { DEFAULT_HOTKEY } from "@/global/stores/main-settings/slices/hotkeys/constants";

interface HotkeyProps {
  id: string,
}

export const Hotkey: React.FC<HotkeyProps> = ({ id }) => {
  const entity = useMainSettingsStore(getHotkey(id))!;

  const controlsActions = {
    editEntity: useMainSettingsStore((state) => state.editHotkey),
    saveEntity: useMainSettingsStore((state) => state.commitHotkey),
    cancelEntity: useMainSettingsStore((state) => state.rollbackHotkey),
    deleteEntity: useMainSettingsStore((state) => state.deleteHotkey),
  };

  return (
    <div className="w-full px-6 border rounded-2xl flex flex-col items-stretch">
      <div className="w-full py-4 flex items-center justify-between">
        <div className="flex grow flex-col justify-center">
          <p className="font-medium">{entity.current.title}</p>
          {entity.current.description && <p className="text-sm text-foreground/60">{entity.current.description}</p>}
        </div>
        <div className="flex gap-2">
          <Controls entity={entity} defaultValue={DEFAULT_HOTKEY} {...controlsActions} />
        </div>
      </div>
      {entity.isEditing && <HotkeyFields id={id} />}
    </div>
  );
};
