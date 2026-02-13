import { createPortal } from "react-dom";

import { useThemeStore } from "@/global/stores/theme";
import { ThemeName } from "@/global/data/themes";

import { getStyles } from "./utils/get-styles";

export const Theme: React.FC = () => {
  const { mode, theme } = useThemeStore();

  const css = getStyles(mode, theme, ThemeName.BoldTech);

  return createPortal(<style type="text/css">{css}</style>, document.head);
};
