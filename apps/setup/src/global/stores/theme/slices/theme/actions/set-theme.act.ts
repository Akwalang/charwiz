import { ThemeName } from "@/global/data/themes";

export const setTheme = (set: any) => (theme: ThemeName) => {
  set(() => ({ theme }));
};
