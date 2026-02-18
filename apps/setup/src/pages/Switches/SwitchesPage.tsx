import { Switch } from "./components/Switch/Switch";

import { PageTitle } from "@/views/components";
import { Button } from "@/views/ui/button";

import { useLang } from "@/global/hooks";

import { useMainSettingsStore } from "@/global/stores/main-settings";
import { getSwitches } from "@/global/stores/main-settings/selectors";

interface SwitchesPageProps {}

export const SwitchesPage: React.FC<SwitchesPageProps> = () => {
  const lang = useLang((state) => state.switches);

  const switches = useMainSettingsStore(getSwitches);
  const createSwitch = useMainSettingsStore((state) => state.createSwitch);

  return (
    <>
      <PageTitle title={lang.page.title} description={lang.page.description} />

      <section className="flex flex-col gap-4">
        {switches.map((item) => <Switch key={item.id} id={item.id} />)}
      </section>

      <div className="mt-6 flex justify-center">
        <Button onClick={createSwitch}>{lang.page.addNewItem}</Button>
      </div>
    </>
  );
};
