import { PageTitle, SettingsItem, ApplySettings } from "@/views/components";

interface HotkeysPageProps {}

export const HotkeysPage: React.FC<HotkeysPageProps> = () => {
  return (
    <>
      <PageTitle title="Hotkey settings" description="Manage global keyboard shortcuts" />

      <section className="mb-10">
        <h2 className="text-sm font-semibold text-gray-400 uppercase mb-4">Global</h2>

        <div className="divide-y border rounded-xl">
          <SettingsItem title="Case Convert" description="Convert text from lower to upper case and back" />
          <SettingsItem title="Case Convert" description="Convert text from lower to upper case and back" />
          <SettingsItem title="Case Convert" description="Convert text from lower to upper case and back" />
          <SettingsItem title="Case Convert" description="Convert text from lower to upper case and back" />
        </div>
      </section>

      <section className="mb-10">
        <h2 className="text-sm font-semibold text-gray-400 uppercase mb-4">Global</h2>

        <div className="divide-y border rounded-xl">

          <div className="py-4 px-6 flex items-center justify-between">
            <div>
              <p className="font-medium">Switch layout</p>
              <p className="text-sm text-gray-500">Global hotkey</p>
            </div>

            <div className="flex items-center gap-6">
              <div className="inline-flex items-center gap-2">
                <span className="key">Ctrl</span>
                <span className="text-gray-400">+</span>
                <span className="key">Shift</span>
                <span className="text-gray-400">+</span>
                <span className="key">Space</span>
              </div>

              <button className="text-sm text-indigo-600 hover:underline">
                Change
              </button>
            </div>
          </div>

          <div className="py-4 px-6 flex items-center justify-between">
            <div>
              <p className="font-medium">Invert case</p>
              <p className="text-sm text-gray-500">Selected text</p>
            </div>

            <div className="flex items-center gap-6">
              <div className="inline-flex items-center gap-2">
                <span className="key">Ctrl</span>
                <span className="text-gray-400">+</span>
                <span className="key">Alt</span>
                <span className="text-gray-400">+</span>
                <span className="key">I</span>
              </div>

              <button className="text-sm text-indigo-600 hover:underline">
                Change
              </button>
            </div>
          </div>

        </div>
      </section>

      <section className="mb-10">
        <h2 className="text-sm font-semibold text-gray-400 uppercase mb-4">Text Transform</h2>

        <div className="divide-y border rounded-xl">

          <div className="py-4 px-6 flex items-center justify-between">
            <p className="font-medium">snake_case</p>
            <div className="inline-flex items-center gap-2">
              <span className="key">Ctrl</span>
              <span className="text-gray-400">+</span>
              <span className="key">1</span>
            </div>
          </div>

          <div className="py-4 px-6 flex items-center justify-between">
            <p className="font-medium">camelCase</p>
            <div className="inline-flex items-center gap-2">
              <span className="key">Ctrl</span>
              <span className="text-gray-400">+</span>
              <span className="key">2</span>
            </div>
          </div>

          <div className="py-4 px-6 flex items-center justify-between">
            <p className="font-medium">kebab-case</p>
            <div className="inline-flex items-center gap-2">
              <span className="key">Ctrl</span>
              <span className="text-gray-400">+</span>
              <span className="key">3</span>
            </div>
          </div>

        </div>
      </section>

      <ApplySettings />
    </>
  );
};
