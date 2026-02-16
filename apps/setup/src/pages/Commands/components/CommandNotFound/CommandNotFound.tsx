import { useLang } from "@/global/hooks";

interface CommandNotFoundProps {}

export const CommandNotFound: React.FC<CommandNotFoundProps> = () => {
  const lang = useLang((state) => state.commands);
  
  return (
    <div className="p-2 divide-y border rounded-xl text-center">
      {lang.page.commandNotFound}
    </div>
  );
};
