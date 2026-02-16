import { SettingsSetup } from "@/views/components";

import { useLang } from "@/global/hooks";

import { useMainSettingsStore } from "@/global/stores/main-settings";
import { getHotkey } from "@/global/stores/main-settings/selectors";

interface HotkeyBlockProps {
  id: string,
}

export const HotkeySetup: React.FC<HotkeyBlockProps> = ({ id }) => {
  const lang = useLang((state) => state.hotkeys.page);
  
  const hotkey = useMainSettingsStore(getHotkey(id));

  const editHotkey = useMainSettingsStore((state) => state.editHotkey);
  const saveHotkey = useMainSettingsStore((state) => state.commitHotkey);
  const updateHotkey = useMainSettingsStore((state) => state.updateHotkey);
  const cancelHotkey = useMainSettingsStore((state) => state.rollbackHotkey);
  const deleteHotkey = useMainSettingsStore((state) => state.deleteHotkey);

  if (!hotkey) {
    return (
      <div className="p-2 divide-y border rounded-xl text-center">
        {lang.hotkeyNotFound}
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
