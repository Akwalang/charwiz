import { SettingsSetup } from "@/views/components";

import { useMainSettingsStore } from "@/global/stores/main-settings";
import { getHotkey } from "@/global/stores/main-settings/selectors";

interface HotkeyBlockProps {
  id: string,
}

export const HotkeyBlock: React.FC<HotkeyBlockProps> = ({ id }) => {
  const hotkey = useMainSettingsStore(getHotkey(id));

  const editHotkey = useMainSettingsStore((state) => state.editHotkey);
  const saveHotkey = useMainSettingsStore((state) => state.commitHotkey);
  const updateHotkey = useMainSettingsStore((state) => state.updateHotkey);
  const cancelHotkey = useMainSettingsStore((state) => state.rollbackHotkey);
  const deleteHotkey = useMainSettingsStore((state) => state.deleteHotkey);

  if (!hotkey) {
    return (
      <div className="divide-y border rounded-xl text-center">
        Hotkey setup not found
      </div>
    );
  }

  return (
    <div className="divide-y border rounded-xl">
      <SettingsSetup
        id={id}
        title={hotkey.current.title}
        description={hotkey.current.description}
        values={hotkey.editing}
        isEditing={hotkey.isEditing}
        onEdit={editHotkey}
        onSave={saveHotkey}
        onUpdate={updateHotkey}
        onCancel={cancelHotkey}
        onDelete={deleteHotkey}
      />
    </div>
  );
};
