import { LangCode } from "@/global/data/langs/enums";

export const setLang = (set: any) => (lang: LangCode) => {
  set(() => ({ lang }));
};
