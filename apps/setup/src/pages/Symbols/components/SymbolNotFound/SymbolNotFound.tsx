import { useLang } from "@/global/hooks";

interface SymbolNotFoundProps {}

export const SymbolNotFound: React.FC<SymbolNotFoundProps> = () => {
  const lang = useLang((state) => state.symbols);
  
  return (
    <div className="p-2 divide-y border rounded-xl text-center">
      {lang.page.symbolNotFound}
    </div>
  );
};
