import { HotkeyItem } from "./components/HotkeyItem/HotkeyItem";

import { PageTitle, ApplySettings } from "@/views/components";
import { Button } from "@/views/ui/button";

import { useLang } from "@/global/hooks";

import { useMainSettingsStore } from "@/global/stores/main-settings";
import { getHotkeys } from "@/global/stores/main-settings/selectors";

interface HotkeysPageProps {}

export const HotkeysPage: React.FC<HotkeysPageProps> = () => {
  const lang = useLang((state) => state.hotkeys);

  const hotkeys = useMainSettingsStore(getHotkeys);
  const createHotkey = useMainSettingsStore((state) => state.createHotkey);

  return (
    <>
      <PageTitle title={lang.page.title} description={lang.page.description} />

      <section className="flex flex-col gap-4">
        {hotkeys.map((item) => <HotkeyItem key={item.id} id={item.id} />)}
      </section>

      <div className="mt-6 flex justify-center">
        <Button onClick={createHotkey}>{lang.page.addNewItem}</Button>
      </div>

      <ApplySettings />
    </>
  );
};
