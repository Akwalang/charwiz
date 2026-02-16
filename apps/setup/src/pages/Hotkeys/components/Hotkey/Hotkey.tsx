import { Controls } from "../Form/Controls";
import { HotkeyFields } from "../HotkeyFields/HotkeyFields";

import { useMainSettingsStore } from "@/global/stores/main-settings";
import { getHotkey } from "@/global/stores/main-settings/selectors";

interface HotkeyProps {
  id: string,
}

export const Hotkey: React.FC<HotkeyProps> = ({ id }) => {
  const hotkey = useMainSettingsStore(getHotkey(id))!;

  return (
    <div className="w-full px-6 border rounded-2xl flex flex-col items-stretch">
      <div className="w-full py-4 flex items-center justify-between">
        <div className="flex grow flex-col justify-center">
          <p className="font-medium">{hotkey.current.title}</p>
          {hotkey.current.description &&
            <p className="text-sm text-foreground/60">{hotkey.current.description}</p>
          }
        </div>
        <div className="flex gap-2">
          <Controls id={id} isEditing={hotkey.isEditing} />
        </div>
      </div>
      {hotkey.isEditing && <HotkeyFields id={id} />}
    </div>
  )
};
