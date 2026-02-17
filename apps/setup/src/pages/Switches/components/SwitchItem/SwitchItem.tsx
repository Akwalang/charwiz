import { useMainSettingsStore } from "@/global/stores/main-settings";
import { isSwitchExists } from "@/global/stores/main-settings/selectors";

import { Switch } from "../Switch/Switch";
import { SwitchNotFound } from "../SwitchNotFound/SwitchNotFound";

interface SwitchItemProps {
  id: string,
}

export const SwitchItem: React.FC<SwitchItemProps> = ({ id }) => {
  const isExists = useMainSettingsStore(isSwitchExists(id));

  return isExists ? <Switch id={id} /> : <SwitchNotFound />;
};
