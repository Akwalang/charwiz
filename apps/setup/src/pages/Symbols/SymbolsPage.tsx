import { Symbol } from "./components/Symbol/Symbol";

import { PageTitle } from "@/views/components";
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
        {symbols.map(({ id }) => <Symbol key={id} id={id} />)}
      </section>

      <div className="mt-6 flex justify-center">
        <Button onClick={createSymbol}>{lang.page.addNewItem}</Button>
      </div>
    </>
  );
};
