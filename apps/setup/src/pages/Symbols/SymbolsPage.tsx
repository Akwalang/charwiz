import { SymbolItem } from "./components/SymbolItem/SymbolItem";

import { PageTitle, ApplySettings } from "@/views/components";
import { Button } from "@/views/ui/button";

import { useLang } from "@/global/hooks";

import { useMainSettingsStore } from "@/global/stores/main-settings";
import { getSymbols } from "@/global/stores/main-settings/selectors";

interface SymbolsPageProps {}

export const SymbolsPage: React.FC<SymbolsPageProps> = () => {
  const lang = useLang((state) => state.symbols);

  const symbols = useMainSettingsStore(getSymbols);
  const createSymbol = useMainSettingsStore((state) => state.createSymbol);

  return (
    <>
      <PageTitle title={lang.page.title} description={lang.page.description} />

      <section className="flex flex-col gap-4">
        {symbols.map((item) => <SymbolItem key={item.id} id={item.id} />)}
      </section>

      <div className="mt-6 flex justify-center">
        <Button onClick={createSymbol}>{lang.page.addNewItem}</Button>
      </div>

      <ApplySettings />
    </>
  );
};
