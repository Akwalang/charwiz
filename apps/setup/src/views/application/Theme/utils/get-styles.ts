import { ThemeName, ThemeMode, ThemeItem, themes } from "@/global/data/themes";

export const getStyles = (
  type: ThemeMode,
  name: ThemeName,
  fallback: ThemeName,
): string => {
  let theme: ThemeItem | null = null;

  theme ||= themes[name];
  theme ||= themes[fallback];

  if (!theme) return '';

  return `:root {${theme.styles[type]}}`;
}
