import { useMainSettingsStore } from "@/global/stores/main-settings";
import { isSymbolExists } from "@/global/stores/main-settings/selectors";

import { Symbol } from "../Symbol/Symbol";
import { SymbolNotFound } from "../SymbolNotFound/SymbolNotFound";

interface SymbolItemProps {
  id: string,
}

export const SymbolItem: React.FC<SymbolItemProps> = ({ id }) => {
  const isExists = useMainSettingsStore(isSymbolExists(id));

  return isExists ? <Symbol id={id} /> : <SymbolNotFound />;
};
