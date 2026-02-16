import { useMainSettingsStore } from "@/global/stores/main-settings";
import { isHotkeyExists } from "@/global/stores/main-settings/selectors";

import { Hotkey } from "../Hotkey/Hotkey";
import { HotkeyNotFound } from "../HotkeyNotFound/HotkeyNotFound";

interface HotkeyItemProps {
  id: string,
}

export const HotkeyItem: React.FC<HotkeyItemProps> = ({ id }) => {
  const isExists = useMainSettingsStore(isHotkeyExists(id));

  return isExists ? <Hotkey id={id} /> : <HotkeyNotFound />;
};
