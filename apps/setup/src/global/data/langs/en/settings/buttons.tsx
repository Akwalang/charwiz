import { ButtonsSettings } from "@/global/types/lang/settings/buttons";

export const buttons: ButtonsSettings = {
  edit: "Edit",
  save: "Save",
  cancel: "Cancel",
  delete: "Delete",
  waiting: (remain: number) => <>Waiting... <span className="inline-block w-[12px] text-center">{remain}</span></>,
};
