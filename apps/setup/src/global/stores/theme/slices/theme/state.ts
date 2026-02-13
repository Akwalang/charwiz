import { ThemeName, ThemeMode } from "@/global/data/themes";

export type ThemeState = {
  theme: ThemeName;
  mode: ThemeMode;
};

export const ThemeState = (): ThemeState => ({
  theme: ThemeName.Claymorphism,
  mode: ThemeMode.Light,
}) as const;
