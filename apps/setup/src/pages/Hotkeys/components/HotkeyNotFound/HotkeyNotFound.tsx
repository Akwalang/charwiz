import { useLang } from "@/global/hooks";

interface HotkeyNotFoundProps {}

export const HotkeyNotFound: React.FC<HotkeyNotFoundProps> = () => {
  const lang = useLang((state) => state.hotkeys);
  
  return (
    <div className="p-2 divide-y border rounded-xl text-center">
      {lang.page.hotkeyNotFound}
    </div>
  );
};
