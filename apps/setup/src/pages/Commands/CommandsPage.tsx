import { Command } from "./components/Command/Command";

import { PageTitle } from "@/views/components";
import { Button } from "@/views/ui/button";

import { useLang } from "@/global/hooks";

import { useMainSettingsStore } from "@/global/stores/main-settings";
import { getCommands } from "@/global/stores/main-settings/selectors";

interface CommandsPageProps {}

export const CommandsPage: React.FC<CommandsPageProps> = () => {
  const lang = useLang((state) => state.commands);

  const commands = useMainSettingsStore(getCommands);
  const createCommand = useMainSettingsStore((state) => state.createCommand);

  return (
    <>
      <PageTitle title={lang.page.title} description={lang.page.description} />

      <section className="flex flex-col gap-4">
        {commands.map((item) => <Command key={item.id} id={item.id} />)}
      </section>

      <div className="mt-6 flex justify-center">
        <Button onClick={createCommand}>{lang.page.addNewItem}</Button>
      </div>
    </>
  );
};
