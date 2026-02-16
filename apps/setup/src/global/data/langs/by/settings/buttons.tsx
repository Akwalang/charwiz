import { ButtonsSettings } from "@/global/types/lang/settings/buttons";

export const buttons: ButtonsSettings = {
  edit: "Рэдагаваць",
  save: "Захаваць",
  cancel: "Адмяніць",
  delete: "Выдаліць",
  waiting: (remain: number) => <>Чаканне... <span className="inline-block w-[12px] text-center">{remain}</span></>,
};
