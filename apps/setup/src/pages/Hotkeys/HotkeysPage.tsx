import { HotkeyBlock } from "./components/HotkeyBlock/HotkeyBlock";

import { PageTitle, ApplySettings } from "@/views/components";
import { Button } from "@/views/ui/button";

import { useMainSettingsStore } from "@/global/stores/main-settings";
import { getHotkeys } from "@/global/stores/main-settings/selectors";

interface HotkeysPageProps {}

export const HotkeysPage: React.FC<HotkeysPageProps> = () => {
  const hotkeys = useMainSettingsStore(getHotkeys);

  const createHotkey = useMainSettingsStore((state) => state.createHotkey);

  return (
    <>
      <PageTitle title="Hotkey settings" description="Manage global keyboard shortcuts" />

      <section className="flex flex-col gap-4">
        {hotkeys.map((item) => <HotkeyBlock key={item.id} id={item.id} />)}
      </section>

      <div className="mt-6 flex justify-center">
        <Button onClick={createHotkey}>Add new item</Button>
      </div>

      <ApplySettings />
    </>
  );
};
