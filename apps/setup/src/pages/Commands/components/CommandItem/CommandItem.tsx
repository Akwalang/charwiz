import { useMainSettingsStore } from "@/global/stores/main-settings";
import { isCommandExists } from "@/global/stores/main-settings/selectors";

import { Command } from "../Command/Command";
import { CommandNotFound } from "../CommandNotFound/CommandNotFound";

interface CommandItemProps {
  id: string,
}

export const CommandItem: React.FC<CommandItemProps> = ({ id }) => {
  const isExists = useMainSettingsStore(isCommandExists(id));

  return isExists ? <Command id={id} /> : <CommandNotFound />;
};
