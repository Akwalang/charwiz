import { useLang } from "@/global/hooks";

interface SwitchNotFoundProps {}

export const SwitchNotFound: React.FC<SwitchNotFoundProps> = () => {
  const lang = useLang((state) => state.switches);
  
  return (
    <div className="p-2 divide-y border rounded-xl text-center">
      {lang.page.switchNotFound}
    </div>
  );
};
